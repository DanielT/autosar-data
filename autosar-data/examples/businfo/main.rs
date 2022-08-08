use std::env;

use autosar_data::{AutosarProject, CharacterData, Element, ElementName, EnumItem};
use rustc_hash::FxHashMap;

enum TimeRangeTolerance {
    Relative(i64),
    Absolute(f64),
}

struct TimeRange {
    tolerance: Option<TimeRangeTolerance>,
    value: f64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <arxml filename> [<arxml filename2> ...]", args[0]);
        return;
    }

    let project = AutosarProject::new();

    for filename in &args[1..] {
        if let Err(err) = project.load_arxml_file(filename, false) {
            println!("parsing failed: {err}");
            return;
        }
    }

    extract_bus_info(project);
}

// the starting point for the info we want is a CLUSTER, which is found by iterating over all identifiable elements
// <AUTOSAR> ...
//   <AR-PACKAGE> ...
//     <CAN_CLUSTER> -or- <FLEXRAY-CLUSTER> -or- <J1939-CLUSTER>
fn extract_bus_info(project: AutosarProject) {
    for (_, weak_element) in project.identifiable_elements() {
        let element = weak_element.upgrade().unwrap();
        match element.element_name() {
            ElementName::CanCluster => {
                display_can_cluster(&element);
            }
            ElementName::EthernetCluster => {
                println!("display of ethernet clusters is not implemented");
            }
            ElementName::FlexrayCluster => {
                display_flexray_cluster(&element);
            }
            ElementName::J1939Cluster => {
                display_j1939_cluster(&element);
            }
            ElementName::LinCluster => {
                println!("display of lin clusters is not implemented");
            }
            ElementName::TtcanCluster => {
                println!("display of TTCAN clusters is not implemented");
            }
            _ => {}
        }
    }
}

// display the cluster information for a can cluster
// The can cluster information looks like this
// <CAN-CLUSTER>
//   <SHORT-NAME>Can_Cluster</SHORT-NAME>
//   <CAN-CLUSTER-VARIANTS>
//     <CAN-CLUSTER-CONDITIONAL>
//       <BAUDRATE>500000</BAUDRATE>
//       <PHYSICAL-CHANNELS>
//         <CAN-PHYSICAL-CHANNEL>
//           <SHORT-NAME>Can_Physical_Channel</SHORT-NAME>
//           <FRAME-TRIGGERINGS>
//             ...
//           </FRAME-TRIGGERINGS>
//           ...
//         </CAN-PHYSICAL-CHANNEL>
//       </PHYSICAL-CHANNELS>
//       <CAN-FD-BAUDRATE>2000000</CAN-FD-BAUDRATE>
fn display_can_cluster(cluster_element: &Element) -> Option<()> {
    let cluster_name = cluster_element.item_name()?;

    let ccc = cluster_element
        .get_sub_element(ElementName::CanClusterVariants)
        .and_then(|ccv| ccv.get_sub_element(ElementName::CanClusterConditional))?;
    println!("Can Cluster: {cluster_name}");
    if let Some(baudrate) = ccc
        .get_sub_element(ElementName::Baudrate)
        .and_then(|elem| elem.character_data())
    {
        println!("  Baudrate: {}", baudrate);
    }
    if let Some(baudrate) = ccc
        .get_sub_element(ElementName::CanFdBaudrate)
        .and_then(|elem| elem.character_data())
    {
        println!("  FD Baudrate: {}", baudrate);
    }

    for phys_channel in ccc.get_sub_element(ElementName::PhysicalChannels).map(|elem| {
        elem.sub_elements()
            .filter(|se| se.element_name() == ElementName::CanPhysicalChannel)
    })? {
        println!("  Can Physical Channel: {}", phys_channel.item_name()?);
        let frame_triggerings = phys_channel.get_sub_element(ElementName::FrameTriggerings)?;
        for ft in frame_triggerings.sub_elements() {
            display_can_ft(&ft);
        }
    }

    Some(())
}

// display the cluster information for a J1939 cluster.
// A J1939 cluster is basically a CAN cluster with some extra information.
// The cluster information looks like this:
// <J-1939-CLUSTER>
//   <SHORT-NAME>J1939_Cluster</SHORT-NAME>
//   <J-1939-CLUSTER-VARIANTS>
//     <J-1939-CLUSTER-CONDITIONAL>
//       <BAUDRATE>500000</BAUDRATE>
//       <PHYSICAL-CHANNELS>
//         <CAN-PHYSICAL-CHANNEL>
//           <SHORT-NAME>Can_Physical_Channel</SHORT-NAME>
//           <FRAME-TRIGGERINGS>
//             ...
//           </FRAME-TRIGGERINGS>
//         </CAN-PHYSICAL-CHANNEL>
//       </PHYSICAL-CHANNELS>
//       <CAN-FD-BAUDRATE>2000000</CAN-FD-BAUDRATE>
//       <NETWORK-ID>1</NETWORK-ID>
//       <REQUEST-2-SUPPORT>false</REQUEST-2-SUPPORT>
//       <USES-ADDRESS-ARBITRATION>false</USES-ADDRESS-ARBITRATION>
fn display_j1939_cluster(cluster_element: &Element) -> Option<()> {
    let cluster_name = cluster_element.item_name()?;

    let ccc = cluster_element
        .get_sub_element(ElementName::J1939ClusterVariants)
        .and_then(|ccv| ccv.get_sub_element(ElementName::J1939ClusterConditional))?;
    println!("J1939 Cluster: {cluster_name}");
    if let Some(network_id) = ccc
        .get_sub_element(ElementName::NetworkId)
        .and_then(|elem| elem.character_data())
    {
        println!("  Network id: {}", network_id);
    }
    if let Some(baudrate) = ccc
        .get_sub_element(ElementName::Baudrate)
        .and_then(|elem| elem.character_data())
    {
        println!("  Baudrate: {}", baudrate);
    }
    if let Some(baudrate) = ccc
        .get_sub_element(ElementName::CanFdBaudrate)
        .and_then(|elem| elem.character_data())
    {
        println!("  FD Baudrate: {}", baudrate);
    }
    if let Some(req_2_support) = ccc
        .get_sub_element(ElementName::Request2Support)
        .and_then(|elem| elem.character_data())
    {
        println!("  Request2 support: {}", req_2_support);
    }

    for phys_channel in ccc.get_sub_element(ElementName::PhysicalChannels).map(|elem| {
        elem.sub_elements()
            .filter(|se| se.element_name() == ElementName::CanPhysicalChannel)
    })? {
        println!("  J1939 Physical Channel: {}", phys_channel.item_name()?);
        let frame_triggerings = phys_channel.get_sub_element(ElementName::FrameTriggerings)?;
        for ft in frame_triggerings.sub_elements() {
            display_can_ft(&ft);
        }
    }

    Some(())
}

// Display a CAN frame triggering and triggered frame
// <CAN-FRAME-TRIGGERING>
//   <SHORT-NAME>Can_Frame_Triggering</SHORT-NAME>
//   <FRAME-PORT-REFS>
//     <FRAME-PORT-REF DEST="FRAME-PORT">/Path/To/Some/Frame_Port</FRAME-PORT-REF>
//     ...
//   </FRAME-PORT-REFS>
//   <FRAME-REF DEST="CAN-FRAME">/Path/To/Some/Can_Frame</FRAME-REF>
//   <PDU-TRIGGERINGS>
//     <PDU-TRIGGERING-REF-CONDITIONAL>
//       <PDU-TRIGGERING-REF DEST="PDU-TRIGGERING">/Path/To/Some/Pdu_Triggering</PDU-TRIGGERING-REF>
//     </PDU-TRIGGERING-REF-CONDITIONAL>
//     ...
//   </PDU-TRIGGERINGS>
//   <CAN-ADDRESSING-MODE>STANDARD</CAN-ADDRESSING-MODE>
//   <CAN-FRAME-RX-BEHAVIOR>CAN-20</CAN-FRAME-RX-BEHAVIOR>
//   <CAN-FRAME-TX-BEHAVIOR>CAN-20</CAN-FRAME-TX-BEHAVIOR>
//   <IDENTIFIER>0x123</IDENTIFIER>
// </CAN-FRAME-TRIGGERING>
fn display_can_ft(frame_triggering: &Element) -> Option<()> {
    // get the referenced frame and frame ports first.
    // don't print anything if either of these doesn't exist, because then the frame triggering is non-functional and irrelevant
    let frame = frame_triggering
        .get_sub_element(ElementName::FrameRef)?
        .get_reference_target()
        .ok()?;
    let frame_ports: Vec<Element> = frame_triggering
        .get_sub_element(ElementName::FramePortRefs)?
        .sub_elements()
        .filter(|se| se.element_name() == ElementName::FramePortRef)
        .filter_map(|fpr| fpr.get_reference_target().ok())
        .collect();
    let ft_name = frame_triggering.item_name()?;
    let frame_name = frame.item_name()?;
    // all other elements are optional
    let canid = &frame_triggering
        .get_sub_element(ElementName::Identifier)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| decode_integer(&cdata));
    let addressing_mode = if let Some(CharacterData::Enum(value)) = frame_triggering
        .get_sub_element(ElementName::CanAddressingMode)
        .and_then(|elem| elem.character_data())
    {
        value
    } else {
        // default to standard addressing if the value is missing
        EnumItem::Standard
    };
    let frame_rx_behavior = frame_triggering
        .get_sub_element(ElementName::CanFrameRxBehavior)
        .and_then(|elem| elem.character_data())
        .map(|cdata| cdata.to_string());
    let frame_tx_behavior = frame_triggering
        .get_sub_element(ElementName::CanFrameTxBehavior)
        .and_then(|elem| elem.character_data())
        .map(|cdata| cdata.to_string());
    let (rx_range_lower, rx_range_upper) =
        if let Some(range_elem) = frame_triggering.get_sub_element(ElementName::RxIdentifierRange) {
            (
                range_elem
                    .get_sub_element(ElementName::LowerCanId)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cdata| decode_integer(&cdata)),
                range_elem
                    .get_sub_element(ElementName::UpperCanId)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cdata| decode_integer(&cdata)),
            )
        } else {
            (None, None)
        };

    let (sender_ecus, receiver_ecus) = get_rx_tx_ecus(frame_ports);

    let frame_length = frame
        .get_sub_element(ElementName::FrameLength)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| decode_integer(&cdata));

    print!("    Can Frame: {frame_name}");
    if ft_name != frame_name {
        print!(" (frame triggering: {ft_name})");
    }
    println!("");
    if !sender_ecus.is_empty() {
        println!("      Senders: {}", sender_ecus.join(", "));
    }
    if !receiver_ecus.is_empty() {
        println!("      Receivers: {}", receiver_ecus.join(", "));
    }
    if let Some(tx_behavior) = frame_tx_behavior {
        println!("      Mode: {tx_behavior}")
    } else if let Some(rx_behavior) = frame_rx_behavior {
        println!("      Mode: {rx_behavior}")
    }
    if let Some(canid) = canid {
        println!("      Can Id: 0x{canid:x} ({addressing_mode})");
    }
    if let (Some(lower), Some(upper)) = (rx_range_lower, rx_range_upper) {
        println!("      Rx Id range: 0x{lower:x} - 0x{upper:x}");
    }
    if let Some(length) = frame_length {
        println!("      Length: {length}");
    }

    // the pdu to frame mappings contain references to the PDUs in each frame
    // <CAN-FRAME>
    //   <SHORT-NAME>Can_Frame</SHORT-NAME>
    //   <FRAME-LENGTH>...</FRAME-LENGTH>
    //   <PDU-TO-FRAME-MAPPINGS>
    //     <PDU-TO-FRAME-MAPPING>
    //       <PDU-REF DEST="...">/Path/To/A/Pdu</PDU-REF>
    //       ...
    //     </PDU-TO-FRAME-MAPPING>
    //     ...
    //   </PDU-TO-FRAME-MAPPINGS>
    // </CAN-FRAME>
    if let Some(mappings) = frame.get_sub_element(ElementName::PduToFrameMappings) {
        for pdu_mapping in mappings.sub_elements() {
            display_mapped_pdu(&pdu_mapping);
        }
    }
    println!("");

    Some(())
}

// display the cluster information for a flexray cluster
// The flexray cluster information looks like this:
// <FLEXRAY-CLUSTER>
//   <SHORT-NAME>Flexray_Cluster</SHORT-NAME>
//   <FLEXRAY-CLUSTER-VARIANTS>
//     <FLEXRAY-CLUSTER-CONDITIONAL>
//       <BAUDRATE>10000000</BAUDRATE>
//       <PHYSICAL-CHANNELS>
//         <FLEXRAY-PHYSICAL-CHANNEL>
//           <SHORT-NAME>Flexray_Physical_Channel</SHORT-NAME>
//           <FRAME-TRIGGERINGS>
//             ...
//           </FRAME-TRIGGERINGS>
//         </FLEXRAY-PHYSICAL-CHANNEL>
//       </PHYSICAL-CHANNELS>
//       <PROTOCOL-NAME>FlexRay</PROTOCOL-NAME>
//       <PROTOCOL-VERSION>2.1</PROTOCOL-VERSION>
//       <MACRO-PER-CYCLE>...</MACRO-PER-CYCLE>
//       <NUMBER-OF-MINISLOTS>...</NUMBER-OF-MINISLOTS>
//       <NUMBER-OF-STATIC-SLOTS>...</NUMBER-OF-STATIC-SLOTS>
//       <PAYLOAD-LENGTH-STATIC>...</PAYLOAD-LENGTH-STATIC>
//       <STATIC-SLOT-DURATION>...</STATIC-SLOT-DURATION>
//       ...
fn display_flexray_cluster(cluster_element: &Element) -> Option<()> {
    let cluster_name = cluster_element.item_name()?;

    let fcc = cluster_element
        .get_sub_element(ElementName::FlexrayClusterVariants)
        .and_then(|fcv| fcv.get_sub_element(ElementName::FlexrayClusterConditional))?;
    println!("Flexray Cluster: {cluster_name}");
    if let Some(proto_name) = fcc
        .get_sub_element(ElementName::ProtocolName)
        .and_then(|elem| elem.character_data())
    {
        if let Some(proto_ver) = fcc
            .get_sub_element(ElementName::ProtocolVersion)
            .and_then(|elem| elem.character_data())
        {
            println!("  Protocol: {proto_name} {proto_ver}");
        }
    }
    if let Some(baudrate) = fcc
        .get_sub_element(ElementName::Baudrate)
        .and_then(|elem| elem.character_data())
    {
        println!("  Baudrate: {baudrate}");
    }
    if let Some(macroticks) = fcc
        .get_sub_element(ElementName::MacroPerCycle)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| decode_integer(&cdata))
    {
        println!("  Macroticks per cycle: {macroticks}");
    }
    if let Some(static_slots) = fcc
        .get_sub_element(ElementName::NumberOfStaticSlots)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| decode_integer(&cdata))
    {
        println!("  Number of static slots: {static_slots}");
    }
    if let Some(static_duration) = fcc
        .get_sub_element(ElementName::StaticSlotDuration)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| decode_integer(&cdata))
    {
        println!("  Static slot duration: {static_duration}");
    }
    if let Some(static_length) = fcc
        .get_sub_element(ElementName::PayloadLengthStatic)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| decode_integer(&cdata))
    {
        println!("  Static slot payload length: {static_length}");
    }
    if let Some(minislots) = fcc
        .get_sub_element(ElementName::NumberOfMinislots)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| decode_integer(&cdata))
    {
        println!("  Number of minislots: {minislots}");
    }

    for phys_channel in fcc.get_sub_element(ElementName::PhysicalChannels).map(|elem| {
        elem.sub_elements()
            .filter(|se| se.element_name() == ElementName::FlexrayPhysicalChannel)
    })? {
        println!("  Flexray Physical Channel: {}", phys_channel.item_name()?);
        let frame_triggerings = phys_channel.get_sub_element(ElementName::FrameTriggerings)?;
        for ft in frame_triggerings.sub_elements() {
            display_flexray_ft(&ft);
        }
    }

    Some(())
}

// display a flexray frame triggering
// <FLEXRAY-FRAME-TRIGGERING>
//   <SHORT-NAME>Flexray_Frame_Triggering</SHORT-NAME>
//   <FRAME-PORT-REFS>
//     <FRAME-PORT-REF DEST="FRAME-PORT">/Path/To/Some/Frame_Port</FRAME-PORT-REF>
//     ...
//   </FRAME-PORT-REFS>
//   <FRAME-REF DEST="FLEXRAY-FRAME">/Path/To/Some/Flexray_Frame</FRAME-REF>
//   <PDU-TRIGGERINGS>
//     <PDU-TRIGGERING-REF-CONDITIONAL>
//       <PDU-TRIGGERING-REF DEST="PDU-TRIGGERING">/Path/To/Some/Pdu_Triggering</PDU-TRIGGERING-REF>
//     </PDU-TRIGGERING-REF-CONDITIONAL>
//     ...
//   </PDU-TRIGGERINGS>
//   <ABSOLUTELY-SCHEDULED-TIMINGS>
//     <FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING>
//       <COMMUNICATION-CYCLE>
//         <CYCLE-REPETITION>
//           <BASE-CYCLE>0</BASE-CYCLE>
//           <CYCLE-REPETITION>CYCLE-REPETITION-1</CYCLE-REPETITION>
//         </CYCLE-REPETITION>
//       </COMMUNICATION-CYCLE>
//       <SLOT-ID>22</SLOT-ID>
//     </FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING>
//   </ABSOLUTELY-SCHEDULED-TIMINGS>
//   <ALLOW-DYNAMIC-L-SDU-LENGTH>false</ALLOW-DYNAMIC-L-SDU-LENGTH>
//   <PAYLOAD-PREAMBLE-INDICATOR>false</PAYLOAD-PREAMBLE-INDICATOR>
fn display_flexray_ft(frame_triggering: &Element) -> Option<()> {
    // get the referenced frame and frame ports first.
    // don't print anything if either of these doesn't exist, because then the frame triggering is non-functional and irrelevant
    let frame = frame_triggering
        .get_sub_element(ElementName::FrameRef)?
        .get_reference_target()
        .ok()?;
    let frame_ports: Vec<Element> = frame_triggering
        .get_sub_element(ElementName::FramePortRefs)?
        .sub_elements()
        .filter(|se| se.element_name() == ElementName::FramePortRef)
        .filter_map(|fpr| fpr.get_reference_target().ok())
        .collect();
    let ft_name = frame_triggering.item_name()?;
    let frame_name = frame.item_name()?;

    // all other elements are optional
    let mut slot_id = None;
    let mut base_cycle = None;
    let mut cycle_repetition = None;
    if let Some(timings) = frame_triggering
        .get_sub_element(ElementName::AbsolutelyScheduledTimings)
        .and_then(|elem| elem.get_sub_element(ElementName::FlexrayAbsolutelyScheduledTiming))
    {
        slot_id = timings
            .get_sub_element(ElementName::SlotId)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| decode_integer(&cdata));
        base_cycle = timings
            .get_sub_element(ElementName::CommunicationCycle)
            .and_then(|elem| elem.get_sub_element(ElementName::CycleRepetition))
            .and_then(|elem| elem.get_sub_element(ElementName::BaseCycle))
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| decode_integer(&cdata));
        cycle_repetition = timings
            .get_sub_element(ElementName::CommunicationCycle)
            .and_then(|elem| elem.get_sub_element(ElementName::CycleRepetition))
            .and_then(|elem| elem.get_sub_element(ElementName::CycleRepetition))
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.enum_value());
    }

    let (sender_ecus, receiver_ecus) = get_rx_tx_ecus(frame_ports);

    print!("    Flexray Frame: {frame_name}");
    if ft_name != frame_name {
        print!(" (frame triggering: {ft_name})");
    }
    println!("");
    if !sender_ecus.is_empty() {
        println!("      Senders: {}", sender_ecus.join(", "));
    }
    if !receiver_ecus.is_empty() {
        println!("      Receivers: {}", receiver_ecus.join(", "));
    }
    if let Some(slot_id) = slot_id {
        println!("      Slot Id: {slot_id}");
    }
    if let (Some(base_cycle), Some(cycle_repetition)) = (base_cycle, cycle_repetition) {
        println!("      Communication cycle: {base_cycle} / {cycle_repetition}");
    }

    // the pdu to frame mappings contain references to the PDUs in each frame
    // <FLEXRAY-FRAME>
    //   <SHORT-NAME>Flexray_Frame</SHORT-NAME>
    //   <FRAME-LENGTH>...</FRAME-LENGTH>
    //   <PDU-TO-FRAME-MAPPINGS>
    //     <PDU-TO-FRAME-MAPPING>
    //       <SHORT-NAME>Pdu_To_Frame_Mapping_Name</SHORT-NAME>
    //       <PACKING-BYTE-ORDER>MOST-SIGNIFICANT-BYTE-FIRST</PACKING-BYTE-ORDER>
    //       <PDU-REF DEST="...">/Path/To/A/Pdu</PDU-REF>
    //       <START-POSITION>0</START-POSITION>
    //     </PDU-TO-FRAME-MAPPING>
    //     ...
    //   </PDU-TO-FRAME-MAPPINGS>
    // </FLEXRAY-FRAME>
    if let Some(mappings) = frame.get_sub_element(ElementName::PduToFrameMappings) {
        for pdu_mapping in mappings.sub_elements() {
            display_mapped_pdu(&pdu_mapping);
        }
    }
    println!("");

    Some(())
}

// from a list of <FRAME-PORT-REF>s, determine for each if it is an input or output port and get name of the ECU that owns the port
// return two lists, one each for input and output ECUs
// a frame port loos like this:
// <FRAME-PORT>
//   <SHORT-NAME>Frame_Port_Name</SHORT-NAME>
//   <COMMUNICATION-DIRECTION>IN</COMMUNICATION-DIRECTION> -or- <COMMUNICATION-DIRECTION>OUT</COMMUNICATION-DIRECTION>
// </FRAME-PORT>
fn get_rx_tx_ecus(frame_ports: Vec<Element>) -> (Vec<String>, Vec<String>) {
    let mut sender_ecus = Vec::new();
    let mut receiver_ecus = Vec::new();
    for fp in frame_ports {
        if let Some(CharacterData::Enum(direction)) = fp
            .get_sub_element(ElementName::CommunicationDirection)
            .and_then(|elem| elem.character_data())
        {
            match direction {
                EnumItem::In => {
                    if let Some(name) = ecu_of_frame_port(&fp) {
                        receiver_ecus.push(name);
                    }
                }
                EnumItem::Out => {
                    if let Some(name) = ecu_of_frame_port(&fp) {
                        sender_ecus.push(name);
                    }
                }
                _ => {}
            }
        }
    }
    (sender_ecus, receiver_ecus)
}

// each frame port is a child element of an ECU-INSTANCE
// specifically, the hierarchy looks like this:
// <ECU-INSTANCE>
//   <CONNECTORS>
//     <CAN-COMMUNICATION-CONNECTOR> or <FLEXRAY-COMMUNICATION-CONNECTOR>
//       <ECU-COMM-PORT-INSTANCES>
//         <FRAME-PORT>
// Therefore the ecu name can be found by going up the hierarchy 4 times.
fn ecu_of_frame_port(frame_port: &Element) -> Option<String> {
    let ecu_comm_port_instance = frame_port.parent().ok()??;
    let comm_connector = ecu_comm_port_instance.parent().ok()??;
    let connectors = comm_connector.parent().ok()??;
    let ecu_instance = connectors.parent().ok()??;
    ecu_instance.item_name()
}

// display the info of a <PDU-TO-FRAME-MAPPING>
// <PDU-TO-FRAME-MAPPING>
//   <SHORT-NAME>Pdu_To_Frame_Mapping_Name</SHORT-NAME>
//   <PACKING-BYTE-ORDER>MOST-SIGNIFICANT-BYTE-FIRST</PACKING-BYTE-ORDER>
//   <PDU-REF DEST="...">/Path/To/A/Pdu</PDU-REF>
//   <START-POSITION>0</START-POSITION>
// </PDU-TO-FRAME-MAPPING>
fn display_mapped_pdu(pdu_mapping: &Element) -> Option<()> {
    let pdu = pdu_mapping
        .get_sub_element(ElementName::PduRef)
        .and_then(|pduref| pduref.get_reference_target().ok())?;
    let pdu_name = pdu.item_name()?;
    let byte_order = pdu_mapping
        .get_sub_element(ElementName::PackingByteOrder)
        .and_then(|elem| elem.character_data())
        .map(|cdata| cdata.to_string());
    let start_position = pdu_mapping
        .get_sub_element(ElementName::StartPosition)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| decode_integer(&cdata));
    println!("      Mapped {}: {pdu_name}", pdu.element_name());
    if let Some(bo) = byte_order {
        println!("        Byte order: {bo}");
    }
    if let Some(pos) = start_position {
        println!("        Start position: {pos}");
    }

    display_pdu(&pdu, 0);
    Some(())
}

static SPACES: &str =
"                                                                                                                                ";

// display a PDU
// Since CONTAINER-I-PDUs and SECURED-I-PDUs contain other PDUs this function can be called recursively.
// The indent parameter is used to make sure the recursively printed data is indented correctly
// There are several types of PDUs for different uses; they all share the length parameter.
// Additionally, any PDU that will be sent inside a CONTAINER-I-PDU needs to have <CONTAINED-I-PDU-PROPS>
// with (at least) a header id.
fn display_pdu(pdu: &Element, indent: usize) -> Option<()> {
    let indentation = &SPACES[..(indent * 4) + 8];
    if let Some(length) = pdu
        .get_sub_element(ElementName::Length)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| decode_integer(&cdata))
    {
        println!("{indentation}Length: {length}");
    }
    if let Some(dynamic_length) = pdu
        .get_sub_element(ElementName::HasDynamicLength)
        .and_then(|elem| elem.character_data())
    {
        println!("{indentation}Has dynamic length: {dynamic_length}");
    }
    // contained PDUs have either a short or a long header id. Pdus that are not transmitted in a CONTAINER-I-PDU do not use this.
    // <CONTAINED-I-PDU-PROPS>
    //   <HEADER-ID-SHORT-HEADER>12345</HEADER-ID-SHORT-HEADER>
    // </CONTAINED-I-PDU-PROPS>
    if let Some(contained_header_short_id) = pdu
        .get_sub_element(ElementName::ContainedIPduProps)
        .and_then(|elem| elem.get_sub_element(ElementName::HeaderIdShortHeader))
        .and_then(|elem| elem.character_data())
    {
        println!("{indentation}Contained header id (short): {contained_header_short_id}");
    }
    if let Some(contained_header_long_id) = pdu
        .get_sub_element(ElementName::ContainedIPduProps)
        .and_then(|elem| elem.get_sub_element(ElementName::HeaderIdLongHeader))
        .and_then(|elem| elem.character_data())
    {
        println!("{indentation}Contained header id (long): {contained_header_long_id}");
    }
    // all other info depends on the PDU type
    // some PDU types have no specific information and no additional information is printed
    match pdu.element_name() {
        ElementName::ISignalIPdu => {
            display_isignal_ipdu(pdu, indent);
        }
        ElementName::DcmIPdu => {
            display_dcm_ipdu(pdu, indent);
        }
        ElementName::NmPdu => {
            display_nm_pdu(pdu, indent);
        }
        ElementName::GeneralPurposeIPdu => {}
        ElementName::NPdu => {}
        ElementName::XcpPdu => {}
        ElementName::ContainerIPdu => {
            display_container_ipdu(pdu, indent);
        }
        ElementName::SecuredIPdu => {
            display_secured_ipdu(pdu, indent);
        }
        ElementName::GeneralPurposePdu => {}
        _ => {
            // panic!("unhandled PDU type: {}", pdu.element_name());
        }
    }

    Some(())
}

// Display an I-SIGNAL-I-PDU, which has the following structure
// <I-SIGNAL-I-PDU>
//   <SHORT-NAME>Pdu_Name</SHORT-NAME>
//   <LENGTH>1</LENGTH>
//   <I-PDU-TIMING-SPECIFICATIONS>
//     <I-PDU-TIMING>
//       <TRANSMISSION-MODE-DECLARATION>
//         <TRANSMISSION-MODE-TRUE-TIMING>
//           <CYCLIC-TIMING>
//             <TIME-OFFSET>
//               <VALUE>0.123</VALUE>
//             </TIME-OFFSET>
//             <TIME-PERIOD>
//               <VALUE>0.05</VALUE>
//             </TIME-PERIOD>
//           </CYCLIC-TIMING>
//           <EVENT-CONTROLLED-TIMING>
//             <NUMBER-OF-REPETITIONS>0</NUMBER-OF-REPETITIONS>
//           </EVENT-CONTROLLED-TIMING>
//         </TRANSMISSION-MODE-TRUE-TIMING>
//         ...
//       </TRANSMISSION-MODE-DECLARATION>
//     </I-PDU-TIMING>
//   </I-PDU-TIMING-SPECIFICATIONS>
//   <I-SIGNAL-TO-PDU-MAPPINGS>
//     <I-SIGNAL-TO-I-PDU-MAPPING>
//       <SHORT-NAME>Mapping_Name</SHORT-NAME>
//       <I-SIGNAL-REF DEST="I-SIGNAL">/Path/To/Signal</I-SIGNAL-REF>
//       <PACKING-BYTE-ORDER>MOST-SIGNIFICANT-BYTE-LAST</PACKING-BYTE-ORDER>
//       <START-POSITION>42</START-POSITION>
//       <TRANSFER-PROPERTY>TRIGGERED</TRANSFER-PROPERTY>
//     </I-SIGNAL-TO-I-PDU-MAPPING>
//     <I-SIGNAL-TO-I-PDU-MAPPING>
//       <SHORT-NAME>Signal_Group_Mapping_Name</SHORT-NAME>
//       <I-SIGNAL-GROUP-REF DEST="I-SIGNAL-GROUP">/Path/To/Signal_Group</I-SIGNAL-GROUP-REF>
//       <TRANSFER-PROPERTY>TRIGGERED</TRANSFER-PROPERTY>
//     </I-SIGNAL-TO-I-PDU-MAPPING>
//     ...
//   </I-SIGNAL-TO-PDU-MAPPINGS>
// </I-SIGNAL-I-PDU>
fn display_isignal_ipdu(pdu: &Element, indent: usize) {
    let indentation = &SPACES[..(indent * 4) + 8];
    if let Some(tx_mode_true_timing) = pdu
        .get_sub_element(ElementName::IPduTimingSpecifications)
        .and_then(|elem| elem.get_sub_element(ElementName::IPduTiming))
        .and_then(|elem| elem.get_sub_element(ElementName::TransmissionModeDeclaration))
        .and_then(|elem| elem.get_sub_element(ElementName::TransmissionModeTrueTiming))
    {
        if let Some(cyclic_timing) = tx_mode_true_timing.get_sub_element(ElementName::CyclicTiming) {
            if let Some(TimeRange { tolerance, value }) = cyclic_timing
                .get_sub_element(ElementName::TimePeriod)
                .and_then(|elem| get_time_range(&elem))
            {
                println!("{indentation}Cyclic timing: {value} s");
                match tolerance {
                    Some(TimeRangeTolerance::Absolute(absval)) => {
                        println!("{indentation}Cyclic timing tolerance: {absval} s")
                    }
                    Some(TimeRangeTolerance::Relative(relval)) => {
                        println!("{indentation}Cyclic timing tolerance: {relval} %")
                    }
                    _ => {}
                }
                if let Some(TimeRange { tolerance, value }) = cyclic_timing
                    .get_sub_element(ElementName::TimeOffset)
                    .and_then(|elem| get_time_range(&elem))
                {
                    println!("{indentation}Cyclic timing offset: {value} s");
                    match tolerance {
                        Some(TimeRangeTolerance::Absolute(absval)) => {
                            println!("{indentation}Cyclic timing offset tolerance: {absval} s")
                        }
                        Some(TimeRangeTolerance::Relative(relval)) => {
                            println!("{indentation}Cyclic timing offset tolerance: {relval} %")
                        }
                        _ => {}
                    }
                }
            }
        }
        if let Some(event_timing) = tx_mode_true_timing.get_sub_element(ElementName::EventControlledTiming) {
            println!("{indentation}Event controlled timing:");
            if let Some(num_reps) = event_timing
                .get_sub_element(ElementName::NumberOfRepetitions)
                .and_then(|elem| elem.character_data())
                .and_then(|cdata| decode_integer(&cdata))
            {
                println!("{indentation}  Number of repetitions: {num_reps}");
            }
            if let Some(repetition_period) = event_timing.get_sub_element(ElementName::RepetitionPeriod) {
                if let Some(TimeRange { tolerance, value }) = get_time_range(&repetition_period) {
                    println!("          Repetition period: {value}");
                    if let Some(tol) = tolerance {
                        match tol {
                            TimeRangeTolerance::Relative(percent) => {
                                println!("{indentation}  Repetition period tolerance: {percent}%")
                            }
                            TimeRangeTolerance::Absolute(abstol) => {
                                println!("{indentation}  Repetition period tolerance: {abstol} s")
                            }
                        }
                    }
                }
            }
        }
    }
    let mut signals = FxHashMap::<String, (String, Option<i64>, Option<i64>)>::default();
    let mut signal_groups = Vec::new();
    if let Some(isignal_to_pdu_mappings) = pdu.get_sub_element(ElementName::ISignalToPduMappings) {
        // collect information about the signals and signal groups
        for mapping in isignal_to_pdu_mappings.sub_elements() {
            if let Some(signal) = mapping
                .get_sub_element(ElementName::ISignalRef)
                .and_then(|elem| elem.get_reference_target().ok())
            {
                // collect signal information
                let refpath = mapping
                    .get_sub_element(ElementName::ISignalRef)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cdata| cdata.string_value())
                    .unwrap();
                let name = signal.item_name().unwrap();
                let start_pos = mapping
                    .get_sub_element(ElementName::StartPosition)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cdata| decode_integer(&cdata));
                let length = signal
                    .get_sub_element(ElementName::Length)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cdata| decode_integer(&cdata));
                signals.insert(refpath, (name, start_pos, length));
            } else if let Some(signal_group) = mapping
                .get_sub_element(ElementName::ISignalGroupRef)
                .and_then(|elem| elem.get_reference_target().ok())
            {
                // store the signal group for now
                signal_groups.push(signal_group);
            }
        }
    }
    // display information about the signal groups
    for signal_group in &signal_groups {
        // any signal that is referenced by the group is removed from the overall set of signals
        display_isignal_group(signal_group, &mut signals, indentation);
    }

    // display any remaining signals that are not part of a signal group
    let mut remaining_signals: Vec<(String, Option<i64>, Option<i64>)> = signals.values().cloned().collect();
    if !remaining_signals.is_empty() {
        if signal_groups.is_empty() {
            println!("{indentation}Signals:");
        } else {
            println!("{indentation}Signals (ungrouped):");
        }
        remaining_signals.sort_by(|a, b| a.1.cmp(&b.1));
        for (name, start_pos, length) in remaining_signals {
            print!("{indentation}  {name}");
            if let Some(start_pos) = start_pos {
                print!(", start pos: {start_pos}");
            }
            if let Some(length) = length {
                print!(", length: {length} bit");
            }
            println!("");
        }
    }
}

// get the timing information for CYCLIC-TIMING or EVENT-CONTROLLED-TIMING
// Both use the same data elements, where a value and a tolerance are defined together
fn get_time_range(base: &Element) -> Option<TimeRange> {
    let value = base
        .get_sub_element(ElementName::Value)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| cdata.double_value())?;

    let tolerance = if let Some(absolute_tolerance) = base
        .get_sub_element(ElementName::AbsoluteTolerance)
        .and_then(|elem| elem.get_sub_element(ElementName::Absolute))
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| cdata.double_value())
    {
        Some(TimeRangeTolerance::Absolute(absolute_tolerance))
    } else if let Some(relative_tolerance) = base
        .get_sub_element(ElementName::RelativeTolerance)
        .and_then(|elem| elem.get_sub_element(ElementName::Relative))
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| decode_integer(&cdata))
    {
        Some(TimeRangeTolerance::Relative(relative_tolerance))
    } else {
        None
    };

    Some(TimeRange { tolerance, value })
}

// Display various relevant elements in an I-SIGNAL-GROUP
// <I-SIGNAL-GROUP>
//   <COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS>
//     <DATA-TRANSFORMATION-REF-CONDITIONAL>
//       <DATA-TRANSFORMATION-REF DEST="DATA-TRANSFORMATION">/Path/To/Data_Transformation</DATA-TRANSFORMATION-REF>
//     </DATA-TRANSFORMATION-REF-CONDITIONAL>
//     ...
//   </COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS>
//   <I-SIGNAL-REFS>
//     <I-SIGNAL-REF DEST="I-SIGNAL">/Path/To/Some/Signal</I-SIGNAL-REF>
//     ...
//   </I-SIGNAL-REFS>
//   <TRANSFORMATION-I-SIGNAL-PROPSS>
//     <END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS>
//       <END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS>
//         <END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL>
//           <TRANSFORMER-REF DEST="TRANSFORMATION-TECHNOLOGY">/Path/to/Transformer</TRANSFORMER-REF>
//           <DATA-IDS>
//             <DATA-ID>123</DATA-ID>
//           </DATA-IDS>
//           <DATA-LENGTH>123</DATA-LENGTH>
//         </END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL>
//       </END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS>
//     </END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS>
//     ...
//   </TRANSFORMATION-I-SIGNAL-PROPSS>
//   ...
fn display_isignal_group(
    signal_group: &Element,
    signals: &mut FxHashMap<String, (String, Option<i64>, Option<i64>)>,
    indentation: &str,
) {
    let name = signal_group.item_name().unwrap();
    println!("{indentation}  Signal group: {name}");
    println!("{indentation}    Signals:");
    let mut sig_group_signals = Vec::new();
    // any signal that is referenced by the group is removed from the overall set and added to a list that only contains the signals of this group
    if let Some(isignal_refs) = signal_group.get_sub_element(ElementName::ISignalRefs) {
        for isignal_ref in isignal_refs
            .sub_elements()
            .filter(|elem| elem.element_name() == ElementName::ISignalRef)
        {
            if let Some(CharacterData::String(path)) = isignal_ref.character_data() {
                if let Some(siginfo) = signals.get(&path) {
                    sig_group_signals.push(siginfo.clone());
                    signals.remove(&path);
                }
            }
        }
    }
    // sort and display the group signals
    sig_group_signals.sort_by(|a, b| a.1.cmp(&b.1));
    for (name, start_pos, length) in sig_group_signals {
        print!("{indentation}      {name}");
        if let Some(start_pos) = start_pos {
            print!(", start pos: {start_pos}");
        }
        if let Some(length) = length {
            print!(", length: {length} bit");
        }
        println!("");
    }
    // show minimal info about any data transformation attached to the group
    if let Some(com_transformations) = signal_group.get_sub_element(ElementName::ComBasedSignalGroupTransformations) {
        for elem in com_transformations.sub_elements() {
            if let Some(data_transformation) = elem
                .get_sub_element(ElementName::DataTransformationRef)
                .and_then(|elem| elem.get_reference_target().ok())
            {
                println!(
                    "{indentation}    Data transformation: {}",
                    data_transformation.item_name().unwrap()
                );
            }
        }
    }
    // e2e data transformations have additional properties
    if let Some(transformation_props) = signal_group.get_sub_element(ElementName::TransformationISignalPropss) {
        for e2exf_props in transformation_props
            .sub_elements()
            .filter(|elem| elem.element_name() == ElementName::EndToEndTransformationISignalProps)
        {
            if let Some(e2exf_props_cond) = e2exf_props
                .get_sub_element(ElementName::EndToEndTransformationISignalPropsVariants)
                .and_then(|elem| elem.get_sub_element(ElementName::EndToEndTransformationISignalPropsConditional))
            {
                let transformer_name = e2exf_props_cond
                    .get_sub_element(ElementName::TransformerRef)
                    .and_then(|elem| elem.get_reference_target().ok())
                    .and_then(|elem| elem.item_name());
                let data_id = e2exf_props_cond
                    .get_sub_element(ElementName::DataIds)
                    .and_then(|elem| elem.get_sub_element(ElementName::DataId))
                    .and_then(|elem| elem.character_data())
                    .and_then(|cdata| decode_integer(&cdata));
                let data_length = e2exf_props_cond
                    .get_sub_element(ElementName::DataLength)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cdata| decode_integer(&cdata));
                if let (Some(name), Some(id), Some(length)) = (transformer_name, data_id, data_length) {
                    println!("{indentation}    End to end transformer properties: {name}");
                    println!("{indentation}      Data id: {id}");
                    println!("{indentation}      Length: {length}");
                }
            }
        }
    }
}

// Display the content of a DCM-I-PDU
// <DCM-I-PDU>
//   <SHORT-NAME>Dcm_IPdu</SHORT-NAME>
//   <HAS-DYNAMIC-LENGTH>false</HAS-DYNAMIC-LENGTH>
//   <LENGTH>123</LENGTH>
//   <DIAG-PDU-TYPE>DIAG-REQUEST</DIAG-PDU-TYPE>
// </DCM-I-PDU>
fn display_dcm_ipdu(pdu: &Element, indent: usize) {
    let indentation = &SPACES[..(indent * 4) + 8];
    if let Some(diag_pdu_type) = pdu
        .get_sub_element(ElementName::DiagPduType)
        .and_then(|elem| elem.character_data())
        .map(|cdata| cdata.to_string())
    {
        println!("{indentation}Type: {diag_pdu_type}");
    }
}

// Display the content of an NM-PDU
// <NM-PDU>
//   <SHORT-NAME>Nm_Pdu</SHORT-NAME>
//   <LENGTH>8</LENGTH>
//   <I-SIGNAL-TO-I-PDU-MAPPINGS>
//     <I-SIGNAL-TO-I-PDU-MAPPING>
//       <SHORT-NAME>Nm_Pdu_Signal_Mapping</SHORT-NAME>
//       <I-SIGNAL-REF DEST="I-SIGNAL">/Path/To/Nm/Signal</I-SIGNAL-REF>
//       <PACKING-BYTE-ORDER>OPAQUE</PACKING-BYTE-ORDER>
//       <START-POSITION>8</START-POSITION>
//       <TRANSFER-PROPERTY>PENDING</TRANSFER-PROPERTY>
//     </I-SIGNAL-TO-I-PDU-MAPPING>
//   </I-SIGNAL-TO-I-PDU-MAPPINGS>
//   <NM-DATA-INFORMATION>true</NM-DATA-INFORMATION>
//   <UNUSED-BIT-PATTERN>0</UNUSED-BIT-PATTERN>
// </NM-PDU>
fn display_nm_pdu(pdu: &Element, indent: usize) {
    let indentation = &SPACES[..(indent * 4) + 8];
    if let Some(mapping) = pdu
        .get_sub_element(ElementName::ISignalToIPduMappings)
        .and_then(|elem| elem.get_sub_element(ElementName::ISignalToIPduMapping))
    {
        if let Some(signal) = mapping
            .get_sub_element(ElementName::ISignalRef)
            .and_then(|elem| elem.get_reference_target().ok())
        {
            let name = signal.item_name().unwrap();
            print!("{indentation}Nm-Signal: {name}");
            if let Some(start_pos) = mapping
                .get_sub_element(ElementName::StartPosition)
                .and_then(|elem| elem.character_data())
                .and_then(|cdata| decode_integer(&cdata))
            {
                print!(", start pos: {start_pos}");
            }
            if let Some(length) = signal
                .get_sub_element(ElementName::Length)
                .and_then(|elem| elem.character_data())
                .and_then(|cdata| decode_integer(&cdata))
            {
                print!(", length: {length} bit");
            }
            println!("");
        }
    }
}

// <CONTAINER-I-PDU>
//   <SHORT-NAME>Container_IPdu_Name</SHORT-NAME>
//   <LENGTH>64</LENGTH>
//   <CONTAINED-PDU-TRIGGERING-REFS>
//     <CONTAINED-PDU-TRIGGERING-REF DEST="PDU-TRIGGERING">/Path/To/Pdu/Triggering</CONTAINED-PDU-TRIGGERING-REF>
//     ...
//   </CONTAINED-PDU-TRIGGERING-REFS>
//   <CONTAINER-TIMEOUT>0.123</CONTAINER-TIMEOUT>
//   <CONTAINER-TRIGGER>FIRST-CONTAINED-TRIGGER</CONTAINER-TRIGGER>
//   <HEADER-TYPE>SHORT-HEADER</HEADER-TYPE>
// </CONTAINER-I-PDU>
fn display_container_ipdu(pdu: &Element, indent: usize) {
    let indentation = &SPACES[..(indent * 4) + 8];
    if let Some(header_type) = pdu
        .get_sub_element(ElementName::HeaderType)
        .and_then(|elem| elem.character_data())
    {
        println!("{indentation}Header type: {header_type}");
    }
    if let Some(container_timeout) = pdu
        .get_sub_element(ElementName::ContainerTimeout)
        .and_then(|elem| elem.character_data())
        .and_then(|cdata| cdata.double_value())
    {
        println!("{indentation}Container timeout: {container_timeout}");
    }
    if let Some(container_trigger) = pdu
        .get_sub_element(ElementName::ContainerTrigger)
        .and_then(|elem| elem.character_data())
    {
        println!("{indentation}Container trigger: {container_trigger}");
    }
    if let Some(contained_pdu_refs) = pdu.get_sub_element(ElementName::ContainedPduTriggeringRefs) {
        println!("{indentation}Contained PDUs:");
        for contained_ref in contained_pdu_refs.sub_elements() {
            if let Some(contained_pdu) = contained_ref
                .get_reference_target()
                .ok()
                .and_then(|elem| elem.get_sub_element(ElementName::IPduRef))
                .and_then(|elem| elem.get_reference_target().ok())
            {
                let pdu_name = contained_pdu.item_name().unwrap();
                println!("{indentation}  {}: {pdu_name}", contained_pdu.element_name());
                display_pdu(&contained_pdu, indent + 1);
            }
        }
    }
}

// <SECURED-I-PDU>
//   <SHORT-NAME>Secured_IPdu_Name</SHORT-NAME>
//   <LENGTH>123</LENGTH>
//   <PAYLOAD-REF DEST="PDU-TRIGGERING">/Path/To/Payload/Pdu_Triggering</PAYLOAD-REF>
//   <SECURE-COMMUNICATION-PROPS>
//     <AUTH-ALGORITHM>AES-128</AUTH-ALGORITHM>
//     <AUTH-INFO-TX-LENGTH>24</AUTH-INFO-TX-LENGTH>
//     <DATA-ID>12345</DATA-ID>
//     <FRESHNESS-VALUE-ID>12345</FRESHNESS-VALUE-ID>
//     <USE-FRESHNESS-TIMESTAMP>true</USE-FRESHNESS-TIMESTAMP>
//     ...
//   </SECURE-COMMUNICATION-PROPS>
// </SECURED-I-PDU>
fn display_secured_ipdu(pdu: &Element, indent: usize) {
    let indentation = &SPACES[..(indent * 4) + 8];
    if let Some(secure_comm_props) = pdu.get_sub_element(ElementName::SecureCommunicationProps) {
        println!("{indentation}Secure communication properties:");
        if let Some(algo) = secure_comm_props
            .get_sub_element(ElementName::AuthAlgorithm)
            .and_then(|elem| elem.character_data())
        {
            println!("{indentation}  Authentication algorithm: {algo}");
        }
        if let Some(data_id) = secure_comm_props
            .get_sub_element(ElementName::DataId)
            .and_then(|elem| elem.character_data())
        {
            println!("{indentation}  Authentication data id: {data_id}");
        }
        if let Some(tx_len) = secure_comm_props
            .get_sub_element(ElementName::AuthInfoTxLength)
            .and_then(|elem| elem.character_data())
        {
            println!("{indentation}  Authentication info tx length: {tx_len}");
        }
        if let Some(use_freshness) = secure_comm_props
            .get_sub_element(ElementName::UseFreshnessTimestamp)
            .and_then(|elem| elem.character_data())
        {
            println!("{indentation}  Use freshness timestamp: {use_freshness}");
        }
        if let Some(freshness_value_id) = secure_comm_props
            .get_sub_element(ElementName::FreshnessValueId)
            .and_then(|elem| elem.character_data())
        {
            println!("{indentation}  Freshness value id: {freshness_value_id}");
        }
    }
    println!("{indentation}Secure PDU:");
    if let Some(secured_pdu) = pdu
        .get_sub_element(ElementName::PayloadRef)
        .and_then(|elem| elem.get_reference_target().ok())
        .and_then(|elem| elem.get_sub_element(ElementName::IPduRef))
        .and_then(|elem| elem.get_reference_target().ok())
    {
        let pdu_name = secured_pdu.item_name().unwrap();
        println!("{indentation}  {}: {pdu_name}", secured_pdu.element_name());
        display_pdu(&secured_pdu, indent + 1);
    }
}

// decode a string into an integer
// most integers in arxml files are actually declared as strings in the xsd file, to allow hex (0x) and binary (0b) encoded values
fn decode_integer(cdata: &CharacterData) -> Option<i64> {
    if let CharacterData::String(text) = cdata {
        if text == "0" {
            Some(0)
        } else if text.starts_with("0x") {
            let hexstr = text.strip_prefix("0x").unwrap();
            Some(i64::from_str_radix(hexstr, 16).ok()?)
        } else if text.starts_with("0X") {
            let hexstr = text.strip_prefix("0X").unwrap();
            Some(i64::from_str_radix(hexstr, 16).ok()?)
        } else if text.starts_with("0b") {
            let binstr = text.strip_prefix("0b").unwrap();
            Some(i64::from_str_radix(binstr, 2).ok()?)
        } else if text.starts_with("0B") {
            let binstr = text.strip_prefix("0B").unwrap();
            Some(i64::from_str_radix(binstr, 2).ok()?)
        } else if text.starts_with("0") {
            let octstr = text.strip_prefix("0").unwrap();
            Some(i64::from_str_radix(octstr, 8).ok()?)
        } else {
            Some(text.parse().ok()?)
        }
    } else {
        None
    }
}

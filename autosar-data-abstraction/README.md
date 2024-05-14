# autosar-data-abstraction

***Warning: This is a prototype***

This is an abstraction layer on top of the autosar-data model.

Rather than transforming the element based model into a new form, it only presents a view into the existing model, and provides methods to retrieve and modify the data.

As a result the use of autosar-data-abstraction can be freely mixed with direct manipulation of the model. Allowing the two to be mixed freely is a very important desing goal: autosar-data-abstraction only represents a tiny portion of all possible elements in an Autosar file.

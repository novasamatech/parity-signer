package io.parity.signer.screens.scan.transaction.dynamicderivations


//todo dmitry remove this file when dynamic derivations feature implemented


//My changelist
//Reverted Keyset Items on Keyset details screen to to being grouped by network
//Implemented persisted network filter for Keyset Details screen
//Added UI for "Derivations Preview screen" with QR code
//Added "import" label to Key Set Details screen
//Added "import" section to Public Key Details screen




//Call qrparser_try_decode_qr_sequence
//There is new DynamicDerivations type returned
//Call import_dynamic_derivations with the received payload



//ios
//
//Support dynamically derived account address for operating in the Nova Spektr application
//TODOs
//UI
//
//Add UI for "Derivations Preview screen" with QR code
//Add "import" label to Key Set Details screen
//Add "import" section to Public Key Details screen
//
//Rust integration
//
//Support new qrparser_try_decode_qr_sequence response
//Support import_dynamic_derivations Rust call
//Support ExportAddrsV2 to generate QR code
//Support any API changes to display "import" indicators in existing screens
//
//UX flow
//
//Present "Derivations Preview screen" without QR code
//QR code generation based on ExportAddrsV2 and displaying QR code within "Derivations Preview screen"
//Integrate "import" indicators into existing screens
//

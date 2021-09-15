//
//  Navigator.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.9.2021.
//

//This is a custom navigator to keep this code somewhat similar to what android has
//and implement some simple shallow navigation without pulling legacy or experimental libs

import Foundation

/**
 * Struct to store main navstate of the screen
 */
enum SignerScreen {
    case home
    case keys
    case settings
    case history
}

/**
 * State of transaction progress - flow starts on successful scan
 */
enum TransactionState {
    case none
    case parsing
    case preview
    case password
    case signed
}

/**
 * Modals shown in key management screen
 */
enum KeyManagerModal {
    case none
    case newSeed
    case newKey
    case showKey
    case seedBackup
    case keyDeleteConfirm
}

/**
 * Modals shown in settings screen
 */
enum SettingsModal {
    case none
    case showHistory
    case showSeedManager
    case showNetworkManager
    case showDocument(ShownDocument)
}

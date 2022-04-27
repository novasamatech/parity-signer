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
 * Alerts for showing
 *
 * This might be organized differently i iOS and android
 */
enum SignerAlert: Decodable {
    case Empty
    case Error(MError)
    case Shield
    case Confirm(MConfirm)
}

/**
 * Slightly non-trivial navigation
 * We should keep this to minimum
 */
extension SignerDataModel {
    func pushButton(action: Action, details: String = "", seedPhrase: String = "") {
        //Poor man's mutex; just because it's really managed by UI abstraction
        if actionAvailable {
            /** No returns below or things will stall! **/
            actionAvailable = false
            if let tempActionResult = try? backendAction(action: action, details: details, seedPhrase: seedPhrase)
            {
                actionResult = tempActionResult
            }
            //Boink! debounce is here
            Timer.scheduledTimer(withTimeInterval: debounceTime, repeats: false, block: {_ in self.actionAvailable = true})
            /** Return is allowed again **/
        }
    }
}

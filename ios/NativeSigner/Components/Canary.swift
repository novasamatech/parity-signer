//
//  Canary.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 5.8.2021.
//

import Foundation
import Network

/**
 * This is background network indicator. It will paint the shield icon red and write to history
 * NOTE: This might sometimes crash transaction; it is intended although not defined behavior for now
 */
extension SignerDataModel {
    /**
     * Check if alert was triggered
     */
    func checkAlert() {
        if onboardingDone {
            do {
                let res = try historyGetWarnings(dbname: dbName)
                if res {
                    alert = true
                } else {
                    alert = false
                }
            } catch {
                print("History init failed! This will not do.")
                alert = true
            }
        }
    }

    /**
     * Acknowledge alert and reset it
     */
    func resetAlert() {
        do {
            try historyAcknowledgeWarnings(dbname: dbName)
            checkAlert()
            pushButton(action: .goBack)
        } catch {
            print("History init failed! This will not do.")
            alert = true
        }
    }
}

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
        var err = ExternError()
        if (self.onboardingDone) {
            withUnsafeMutablePointer(to: &err) {err_ptr in
                let res = get_warnings(err_ptr, dbName)
                if (err_ptr.pointee.code == 0) {
                    if res == 1 {
                        self.alert = true
                    } else {
                        self.alert = false
                    }
                } else {
                    print("History init failed! This will not do.")
                    print(String(cString: err_ptr.pointee.message))
                    signer_destroy_string(err_ptr.pointee.message)
                    self.alert = true
                }
            }
        }
    }
    
    /**
     * Acknowledge alert and reset it
     */
    func resetAlert() {
        var err = ExternError()
        withUnsafeMutablePointer(to: &err) {err_ptr in
            acknowledge_warnings(err_ptr, dbName)
            if (err_ptr.pointee.code == 0) {
                self.checkAlert()
            } else {
                print("History init failed! This will not do.")
                print(String(cString: err_ptr.pointee.message))
                signer_destroy_string(err_ptr.pointee.message)
                self.alert = true
            }
        }
    }
}

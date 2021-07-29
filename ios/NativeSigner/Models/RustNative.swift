//
//  RustNative.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 22.7.2021.
//

import Foundation

class DevTestObject: ObservableObject {
    var value: String = ""
    var err = ExternError()
    
    init() {
        self.refresh(input: "")
    }
    
    func refresh(input: String) {
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let res = development_test(err_ptr, input)
        if err_ptr.pointee.code == 0 {
            value = String(cString: res!)
            signer_destroy_string(res!)
        } else {
            value = String(cString: err_ptr.pointee.message)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
}

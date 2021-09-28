//
//  SignerTextInput.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.9.2021.
//

import SwiftUI
import UIKit

/**
 * Ok, sorry to admit but I need this
 * Apple did really lousy job on focusing and return-handling of SwiftUI TextField
 * Thus this crutchbicycle
 *
 * Feature-rich custom text input field to serve as all text input fields in Signer
 * Arguments:
 * text: binding to content
 * focus: binding to autofocus trigger (TODO: this is ugly but works surprisingly well)
 * placeholder: placeholder text for when the field is empty
 * autocapitalization: capitalization rules
 * returnKeyType: visual keyboard return style
 * keyboardType: keyboard type indeed
 * onReturn: generic closure to be executed on pressing return (along with defocusing)
 */
struct SignerTextInput: UIViewRepresentable {
    
    final class Coordinator: NSObject, UITextFieldDelegate {
        let parent: SignerTextInput
        
        init(parent: SignerTextInput) {
            self.parent = parent
        }
        
        func textFieldDidChangeSelection(_ textField: UITextField) {
            parent.text = textField.text ?? ""
            self.parent.focus = false
        }
        
        func textFieldDidEndEditing(_ textField: UITextField) {
            self.parent.focus = false
        }
        
        func updateFocus(textField: UITextField) {
            if parent.focus {
                textField.becomeFirstResponder()
                self.parent.focus = false
            }
        }
        
        func textFieldShouldReturn(_ textField: UITextField) -> Bool {
            self.parent.onReturn()
            textField.resignFirstResponder()
            self.parent.focus = false
            return true
        }
    }
    
    @Binding var text: String
    @Binding var focus: Bool
    var placeholder: String?
    var autocapitalization: UITextAutocapitalizationType
    var returnKeyType: UIReturnKeyType
    var keyboardType: UIKeyboardType
    var onReturn: () -> ()
    
    func makeUIView(context: Context) -> UITextField {
        let textField = UITextField(frame: .zero)
        
        textField.delegate = context.coordinator
        textField.placeholder = placeholder
        textField.returnKeyType = returnKeyType
        textField.keyboardType = keyboardType
        textField.autocorrectionType = .no
        textField.autocapitalizationType = autocapitalization
        textField.font = UIFont.preferredFont(forTextStyle: .largeTitle)
        textField.backgroundColor = UIColor(Color("textFieldColor"))
        textField.textColor = UIColor(Color("textEntryColor"))
        
        return textField
    }
    
    func makeCoordinator() -> Coordinator {
        return Coordinator(parent: self)
    }
  
    
    func updateUIView(_ uiView: UITextField, context: Context) {
        uiView.text = text
        if focus && !uiView.isFirstResponder {
            uiView.becomeFirstResponder()
        }
    }
}

//Preview is pretty useless for this field
/*
struct SignerTextInput_Previews: PreviewProvider {
    static var previews: some View {
        SignerTextInput(text: .constant("text"), focus: .constant(false), placeholder: "placeholder", returnKeyType: .done, keyboardType: .default, onReturn: {})
    }
}
*/

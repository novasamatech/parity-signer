//
//  TransactionCommentInput.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.9.2021.
//

import SwiftUI

struct TransactionCommentInput: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        VStack {
            Text("Comment (not published)")
            TextField("comment", text: $data.comment/*, prompt: Text("enter comment")*/)
                .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/).border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: 1)
        }
    }
}

struct TransactionCommentInput_Previews: PreviewProvider {
    static var previews: some View {
        TransactionCommentInput()
    }
}

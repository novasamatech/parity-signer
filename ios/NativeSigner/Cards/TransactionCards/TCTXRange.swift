//
//  TCTXRange.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.9.2021.
//

import SwiftUI

struct TCTXRange: View {
    var value: TxRange
    var body: some View {
        HStack {
            Text("From")
            Text(value.start)
                .foregroundColor(Color("textMainColor"))
            Text("to")
            Text(value.end).foregroundColor(Color("textMainColor"))
            Text("inclusive?")
            Text(value.inclusive)
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
 struct TCTXRange_Previews: PreviewProvider {
 static var previews: some View {
 TCTXRange()
 }
 }*/

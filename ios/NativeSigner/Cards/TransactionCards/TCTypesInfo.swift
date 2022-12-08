//
//  TCTypesInfo.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTypesInfo: View {
    var content: MTypesInfo
    var body: some View {
        HStack {
            if let identicon = content.typesIdPic {
                Identicon(identicon: identicon.svgPayload)
            }
            TCNamedValueCard(
                name: Localizable.TCName.typesHash.string,
                value: content.typesHash ?? ""
            )
        }
    }
}

struct TCTypesInfo_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            TCTypesInfo(
                content: MTypesInfo(
                    typesOnFile: false,
                    typesHash: "typesHas",
                    typesIdPic: .svg(image: PreviewData.exampleIdenticon)
                )
            )
        }
    }
}

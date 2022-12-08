//
//  SeedPhraseView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/09/2022.
//

import SwiftUI

struct SeedPhraseElementModel: Equatable {
    let position: String
    let word: String
}

struct SeedPhraseViewModel: Equatable {
    let seeds: [SeedPhraseElementModel]

    init(seedPhrase: String) {
        seeds = seedPhrase.components(separatedBy: .whitespaces)
            .enumerated()
            .map { SeedPhraseElementModel(
                position: String($0.offset + 1),
                word: $0.element.trimmingCharacters(in: .whitespacesAndNewlines)
            )
            }
    }
}

/// Component to present seed phrase divided by words along with word position label
/// Layout is fixed to 3 columns
struct SeedPhraseView: View {
    private let viewModel: SeedPhraseViewModel
    private let columns = [
        GridItem(.flexible()),
        GridItem(.flexible()),
        GridItem(.flexible())
    ]

    init(
        viewModel: SeedPhraseViewModel
    ) {
        self.viewModel = viewModel
    }

    var body: some View {
        LazyVGrid(columns: columns, alignment: .center, spacing: 0) {
            ForEach(viewModel.seeds, id: \.position) { seedWord in
                HStack {
                    Text(seedWord.position)
                        .font(Fontstyle.bodyM.crypto)
                        .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                        .padding(.leading, Spacing.extraExtraSmall)
                        .frame(minWidth: Sizes.seedWordPositionWidth, alignment: .trailing)
                    Text(seedWord.word)
                        .font(Fontstyle.bodyM.crypto)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .frame(maxWidth: .infinity, alignment: .leading)
                }
                .padding([.bottom, .top], Spacing.extraSmall)
            }
        }
        .padding(Spacing.medium)
        .containerBackground(CornerRadius.small)
    }
}

struct SeedPhraseView_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            Spacer()
            SeedPhraseView(
                viewModel: PreviewData.seedPhraseViewModel
            )
            .padding(Spacing.large)
            Spacer()
        }
        .background(Asset.backgroundSecondary.swiftUIColor)
        .frame(height: .infinity)
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}

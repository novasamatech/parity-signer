//
//  SeedPhraseView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 19/09/2022.
//

import Combine
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
    @StateObject var viewModel: ViewModel
    @EnvironmentObject var applicationStatePublisher: ApplicationStatePublisher

    private let columns = [
        GridItem(.flexible()),
        GridItem(.flexible()),
        GridItem(.flexible())
    ]
    private let reducedWidthColumn = [
        GridItem(.flexible()),
        GridItem(.flexible())
    ]

    var body: some View {
        LazyVGrid(columns: layout(), alignment: .leading, spacing: 0) {
            ForEach(viewModel.dataModel.seeds, id: \.position) { seedWord in
                HStack(alignment: .center, spacing: Spacing.extraExtraSmall) {
                    Text(seedWord.position)
                        .font(.robotoMonoRegular)
                        .foregroundColor(.textAndIconsDisabled)
                        .frame(minWidth: Sizes.seedWordPositionWidth, alignment: .trailing)
                        .minimumScaleFactor(1)
                        .lineLimit(1)
                    Text(seedWord.word)
                        .font(.robotoMonoBold)
                        .foregroundColor(.textAndIconsSecondary)
                        .minimumScaleFactor(1)
                        .lineLimit(1)
                        .privacySensitive(!viewModel.redactedReason.isEmpty)
                }
                .frame(height: 24)
                .padding([.bottom, .top], Spacing.extraExtraSmall)
            }
        }
        .padding(Spacing.medium)
        .containerBackground(CornerRadius.small)
        .redacted(reason: viewModel.redactedReason)
        .onAppear {
            viewModel.use(applicationStatePublisher: applicationStatePublisher)
        }
    }

    private func layout() -> [GridItem] {
        UIScreen.main.bounds.width == DeviceConstants.compactDeviceWidth ? reducedWidthColumn : columns
    }
}

extension SeedPhraseView {
    final class ViewModel: ObservableObject {
        let dataModel: SeedPhraseViewModel
        private let cancelBag = CancelBag()
        private weak var applicationStatePublisher: ApplicationStatePublisher!
        @Published var redactedReason: RedactionReasons = []

        init(
            dataModel: SeedPhraseViewModel
        ) {
            self.dataModel = dataModel
        }

        func use(applicationStatePublisher: ApplicationStatePublisher) {
            self.applicationStatePublisher = applicationStatePublisher
            applicationStatePublisher.$applicationState.sink { [weak self] updatedValue in
                self?.redactedReason = updatedValue == .inactive ? .privacy : []
            }.store(in: cancelBag)
        }
    }
}

#if DEBUG
    struct SeedPhraseView_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                Spacer()
                SeedPhraseView(
                    viewModel: .init(
                        dataModel: .stub
                    )
                )
                .padding(Spacing.medium)
                Spacer()
            }
            .background(.backgroundSecondary)
            .frame(height: .infinity)
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif

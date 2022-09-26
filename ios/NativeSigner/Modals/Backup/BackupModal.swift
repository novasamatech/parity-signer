//
//  BackupModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/09/2022.
//

import SwiftUI

struct KeySummaryViewModel: Equatable {
    let keyName: String
    let base58: String
}

struct BackupModalViewModel: Equatable {
    let header: KeySummaryViewModel
    let derivedKeys: [DerivedKeyOverviewViewModel]
    let seedPhrase: SeedPhraseViewModel
    let qrCode: QRCodeContainerViewModel
}

struct BackupModal: View {
    @State private var animateBackground: Bool = false
    @Binding private var isShowingBackupModal: Bool
    @StateObject private var snackbar: BottomSnackbarPresentation = .init()
    private let viewModel: BackupModalViewModel

    init(
        isShowingBackupModal: Binding<Bool>,
        viewModel: BackupModalViewModel
    ) {
        _isShowingBackupModal = isShowingBackupModal
        self.viewModel = viewModel
    }

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                animateDismissal()
            },
            animateBackground: $animateBackground,
            ignoredEdges: .bottom,
            content: {
                VStack(alignment: .center, spacing: 0) {
                    // Header with X button
                    HStack {
                        VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                            Text(viewModel.header.keyName)
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                .font(Fontstyle.titleS.base)
                            Text(viewModel.header.base58.truncateMiddle())
                                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                                .font(Fontstyle.bodyM.base)
                        }
                        Spacer()
                        CloseModalButton(action: animateDismissal)
                    }
                    .padding(.leading, Spacing.large)
                    .padding(.trailing, Spacing.medium)
                    Divider()
                        .padding(.top, Spacing.medium)
                    ScrollView {
                        VStack(alignment: .center, spacing: Spacing.medium) {
                            // Seed phrase
                            HStack {
                                Localizable.BackupModal.Label.secret.text
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                    .font(Fontstyle.bodyL.base)
                                Spacer()
                            }
                            SeedPhraseView(viewModel: viewModel.seedPhrase)
                            // Derived Keys
                            HStack {
                                Localizable.BackupModal.Label.derived.text
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                    .font(Fontstyle.bodyL.base)
                                Spacer()
                            }
                            VStack(alignment: .leading, spacing: 0) {
                                ForEach(
                                    viewModel.derivedKeys,
                                    id: \.path
                                ) { derivedKey in
                                    DerivedKeyOverviewRow(derivedKey)
                                    if viewModel.derivedKeys.last != derivedKey {
                                        Divider()
                                    }
                                }
                            }
                            // QR Code container
                            HStack {
                                Localizable.BackupModal.Label.qrCode.text
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                    .font(Fontstyle.bodyL.base)
                                Spacer()
                            }
                            QRCodeContainerView(viewModel: viewModel.qrCode)
                                .fixedSize(horizontal: false, vertical: true)
                                .overlay(
                                    RoundedRectangle(cornerRadius: CornerRadius.medium)
                                        .stroke(Asset.fill12.swiftUIColor, lineWidth: 0.5)
                                )
                            Spacer()
                                .frame(height: Spacing.componentSpacer)
                        }
                        .padding([.leading, .trailing], Spacing.large)
                        .padding(.top, Spacing.medium)
                    }
                }
                .onAppear {
                    snackbar.viewModel = .init(
                        title: Localizable.BackupModal.Label.snackbar.string,
                        style: .info,
                        tapToDismiss: false,
                        countdown: .init(counter: 60, viewModel: .snackbarCountdown, onCompletion: animateDismissal)
                    )
                    snackbar.isSnackbarPresented = true
                }
                .padding(.bottom, -Spacing.medium)
                .bottomSnackbar(snackbar.viewModel, isPresented: $snackbar.isSnackbarPresented, autodismissCounter: 60)
                .padding(.bottom, -Spacing.medium)
            }
        )
    }

    private func animateDismissal() {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: isShowingBackupModal.toggle()
        )
    }
}

struct BackupModal_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            BackupModal(
                isShowingBackupModal: Binding<Bool>.constant(true),
                viewModel: PreviewData.exampleBackupViewModel
            )
        }
        .previewLayout(.sizeThatFits)
        .preferredColorScheme(.dark)
    }
}

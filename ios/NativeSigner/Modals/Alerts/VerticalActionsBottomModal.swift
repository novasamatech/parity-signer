//
//  VerticalActionsBottomModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 21/12/2022.
//

import SwiftUI

struct VerticalActionsBottomModalViewModel {
    let title: String
    let content: String?
    let dismissActionLabel: LocalizedStringKey
    let mainActionLabel: LocalizedStringKey

    static let removeGeneralVerifier = VerticalActionsBottomModalViewModel(
        title: Localizable.Settings.Modal.GeneralVerifier.Label.title.string,
        content: Localizable.Settings.Modal.GeneralVerifier.Label.content.string,
        dismissActionLabel: Localizable.Settings.Modal.GeneralVerifier.Action.cancel.key,
        mainActionLabel: Localizable.Settings.Modal.GeneralVerifier.Action.remove.key
    )
}

struct VerticalActionsBottomModal: View {
    private var viewModel: VerticalActionsBottomModalViewModel
    private let mainAction: () -> Void
    private let dismissAction: () -> Void
    @State private var animateBackground: Bool = false
    @Binding private var isShowingBottomAlert: Bool

    init(
        viewModel: VerticalActionsBottomModalViewModel,
        mainAction: @escaping @autoclosure () -> Void,
        dismissAction: @escaping @autoclosure () -> Void = {}(),
        isShowingBottomAlert: Binding<Bool> = Binding<Bool>.constant(false)
    ) {
        self.viewModel = viewModel
        self.mainAction = mainAction
        self.dismissAction = dismissAction
        _isShowingBottomAlert = isShowingBottomAlert
    }

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal(dismissAction()) },
            animateBackground: $animateBackground,
            safeAreaInsetsMode: .full,
            content: {
                VStack(alignment: .leading, spacing: Spacing.medium) {
                    Text(viewModel.title)
                        .font(PrimaryFont.titleL.font)
                    if let content = viewModel.content {
                        Text(content)
                            .font(PrimaryFont.bodyL.font)
                            .lineSpacing(Spacing.extraExtraSmall)
                            .multilineTextAlignment(.leading)
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                    }
                    VStack {
                        PrimaryButton(
                            action: { animateDismissal(mainAction()) },
                            text: viewModel.mainActionLabel,
                            style: .primaryDestructive()
                        )
                        SecondaryButton(
                            action: animateDismissal(dismissAction()),
                            text: viewModel.dismissActionLabel
                        )
                    }
                    .padding(.top, Spacing.extraSmall)
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding(.top, Spacing.medium)
                .padding(.bottom, Spacing.extraSmall + Spacing.medium)
            }
        )
    }

    private func animateDismissal(_ completion: @escaping @autoclosure () -> Void = {}()) {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: {
                isShowingBottomAlert = false
                completion()
            }()
        )
    }
}

struct VerticalActionsBottomModal_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            VerticalActionsBottomModal(
                viewModel: .removeGeneralVerifier,
                mainAction: {}(),
                isShowingBottomAlert: Binding<Bool>.constant(true)
            )
        }
        .previewLayout(.sizeThatFits)
    }
}

//
//  HorizontalActionsBottomModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 07/09/2022.
//

import SwiftUI

struct HorizontalActionsBottomModalViewModel {
    let title: String
    let content: String
    let dismissActionLabel: LocalizedStringKey
    let mainActionLabel: LocalizedStringKey

    static let forgetKeySet = HorizontalActionsBottomModalViewModel(
        title: Localizable.KeySetsModal.Confirmation.Label.title.string,
        content: Localizable.KeySetsModal.Confirmation.Label.content.string,
        dismissActionLabel: Localizable.KeySetsModal.Confirmation.Action.cancel.key,
        mainActionLabel: Localizable.KeySetsModal.Confirmation.Action.remove.key
    )
}

struct HorizontalActionsBottomModal: View {
    private var viewModel: HorizontalActionsBottomModalViewModel
    private let mainAction: () -> Void
    private let dismissAction: () -> Void
    @State private var animateBackground: Bool = false
    @Binding private var isShowingBottomAlert: Bool

    init(
        viewModel: HorizontalActionsBottomModalViewModel,
        mainAction: @escaping () -> Void,
        dismissAction: @escaping () -> Void = {},
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
            content: {
                VStack(alignment: .center, spacing: Spacing.medium) {
                    Text(viewModel.title)
                        .font(Fontstyle.titleL.base)
                    Text(viewModel.content)
                        .font(Fontstyle.bodyL.base)
                        .lineSpacing(4)
                        .multilineTextAlignment(.center)
                        .padding([.leading, .trailing], Spacing.large)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                    HStack {
                        SecondaryButton(
                            action: animateDismissal(dismissAction()),
                            text: viewModel.dismissActionLabel
                        )
                        PrimaryButton(
                            action: { animateDismissal(mainAction()) },
                            text: viewModel.mainActionLabel,
                            style: .primaryDestructive()
                        )
                    }
                    .padding(.top, Spacing.medium)
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding(.top, Spacing.medium)
                .padding(.bottom, Spacing.extraSmall)
            }
        )
    }

    private func animateDismissal(_ completion: @escaping @autoclosure () -> Void = {}()) {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: {
                isShowingBottomAlert.toggle()
                completion()
            }()
        )
    }
}

struct HorizontalActionsBottomModal_Previews: PreviewProvider {
    static var previews: some View {
        HorizontalActionsBottomModal(
            viewModel: .forgetKeySet,
            mainAction: {},
            isShowingBottomAlert: Binding<Bool>.constant(true)
        )
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
        VStack {
            HorizontalActionsBottomModal(
                viewModel: .forgetKeySet,
                mainAction: {},
                isShowingBottomAlert: Binding<Bool>.constant(true)
            )
            .preferredColorScheme(.light)
            .previewLayout(.sizeThatFits)
        }
        .background(.black)
        .preferredColorScheme(.light)
        .previewLayout(.sizeThatFits)
    }
}

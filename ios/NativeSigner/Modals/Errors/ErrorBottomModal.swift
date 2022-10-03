//
//  ErrorBottomModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 29/09/2022.
//

import SwiftUI

struct ActionModel {
    let label: LocalizedStringKey
    let action: () -> Void
}

struct ErrorBottomModal: View {
    private var viewModel: ErrorBottomModalViewModel
    private let dismissAction: () -> Void
    @State private var animateBackground: Bool = false
    @Binding private var isShowingBottomAlert: Bool

    init(
        viewModel: ErrorBottomModalViewModel,
        dismissAction: @escaping @autoclosure () -> Void = {}(),
        isShowingBottomAlert: Binding<Bool> = Binding<Bool>.constant(false)
    ) {
        self.viewModel = viewModel
        self.dismissAction = dismissAction
        _isShowingBottomAlert = isShowingBottomAlert
    }

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal(dismissAction()) },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading, spacing: Spacing.small) {
                    if let icon = viewModel.icon {
                        HStack(alignment: .center) {
                            Spacer()
                            icon
                            Spacer()
                        }
                        .frame(height: Heights.errorModalIconContainer)
                    }
                    Text(viewModel.title)
                        .font(Fontstyle.titleM.base)
                    Text(viewModel.content)
                        .font(Fontstyle.bodyM.base)
                        .lineSpacing(Spacing.extraExtraSmall)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                    VStack {
                        if let primaryAction = viewModel.primaryAction {
                            PrimaryButton(
                                action: { animateDismissal(primaryAction.action()) },
                                text: primaryAction.label
                            )
                        }
                        if let secondaryAction = viewModel.secondaryAction {
                            SecondaryButton(
                                action: animateDismissal(secondaryAction.action()),
                                text: secondaryAction.label
                            )
                        }
                        if let tertiaryAction = viewModel.tertiaryAction {
                            EmptyButton(
                                action: { animateDismissal(tertiaryAction.action()) },
                                text: tertiaryAction.label
                            )
                        }
                    }
                    .padding(.top, Spacing.medium)
                }
                .padding([.leading, .trailing], Spacing.large)
                .padding([.top, .bottom], Spacing.extraSmall)
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

struct ErrorBottomModal_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            ErrorBottomModal(
                viewModel: .connectivityOn(),
                isShowingBottomAlert: Binding<Bool>.constant(true)
            )
            ErrorBottomModal(
                viewModel: .connectivityWasOn(backAction: {}(), continueAction: {}()),
                isShowingBottomAlert: Binding<Bool>.constant(true)
            )
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}

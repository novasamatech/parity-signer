//
//  ErrorBottomModal.swift
//  Polkadot Vault
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
                        .font(PrimaryFont.titleM.font)
                    if let attributedContent = viewModel.attributedContent {
                        Text(attributedContent)
                            .font(PrimaryFont.bodyM.font)
                            .lineSpacing(Spacing.extraExtraSmall)
                            .foregroundColor(.textAndIconsPrimary)
                    } else {
                        Text(viewModel.content)
                            .font(PrimaryFont.bodyM.font)
                            .lineSpacing(Spacing.extraExtraSmall)
                            .foregroundColor(.textAndIconsSecondary)
                    }
                    if let detailsMessage = viewModel.details {
                        Text(detailsMessage)
                            .foregroundColor(.textAndIconsPrimary)
                            .font(PrimaryFont.bodyL.font)
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .padding(Spacing.medium)
                            .strokeContainerBackground()
                            .padding(.top, Spacing.extraSmall)
                    }
                    if viewModel.steps.count > 1 {
                        VStack(alignment: .leading, spacing: Spacing.small) {
                            ForEach(viewModel.steps, id: \.step) { step in
                                HStack(alignment: .top, spacing: 0) {
                                    Text(step.step)
                                        .foregroundColor(.textAndIconsTertiary)
                                        .frame(width: Spacing.large, alignment: .leading)
                                    Text(step.content)
                                        .lineSpacing(Spacing.extraExtraSmall)
                                }
                            }
                        }
                        .font(PrimaryFont.bodyL.font)
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .padding(Spacing.medium)
                        .strokeContainerBackground()
                        .padding(.top, Spacing.extraSmall)
                    }
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
                                action: animateDismissal(tertiaryAction.action()),
                                text: tertiaryAction.label
                            )
                        }
                    }
                    .padding(.top, Spacing.medium)
                }
                .padding(.horizontal, Spacing.large)
                .padding([.top, .bottom], Spacing.extraSmall)
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

#if DEBUG
    struct ErrorBottomModal_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                // General Error
                ErrorBottomModal(
                    viewModel: .alertError(
                        message: Stubs.stubErrorMessage
                    ),
                    isShowingBottomAlert: Binding<Bool>.constant(true)
                )
                // Key Set Management
                ErrorBottomModal(
                    viewModel: .seedPhraseAlreadyExists(),
                    isShowingBottomAlert: Binding<Bool>.constant(true)
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif

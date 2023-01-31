//
//  OnboardingOverviewView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 30/01/2023.
//

import SwiftUI

struct OnboardingOverviewView: View {
    private enum Constants {
        static let numberOfPages: Int = 4
    }

    @StateObject var viewModel: ViewModel

    var body: some View {
        ZStack {
            LinearGradient(
                gradient:
                Gradient(colors: [Asset.overviewGradientTop.swiftUIColor, Asset.overviewGradientBottom.swiftUIColor]),
                startPoint: .top,
                endPoint: .bottom
            )
            .ignoresSafeArea()
            VStack(spacing: 0) {
                GeometryReader { geometry in
                    PaginatedScrollView(
                        currentPageIndex: $viewModel.currentSelectedPage,
                        itemsAmount: Constants.numberOfPages,
                        itemWidth: geometry.size.width,
                        itemPadding: 0,
                        pageWidth: geometry.size.width
                    ) {
                        FeatureOverviewView(
                            text: Localizable.OnboardingOverview.Label.step1.text,
                            image: Asset.overviewPage1.swiftUIImage
                        )
                        FeatureOverviewView(
                            text: Localizable.OnboardingOverview.Label.step2.text,
                            image: Asset.overviewPage2.swiftUIImage
                        )
                        FeatureOverviewView(
                            text: Localizable.OnboardingOverview.Label.step3.text,
                            image: Asset.overviewPage3.swiftUIImage
                        )
                        FeatureOverviewView(
                            text: Localizable.OnboardingOverview.Label.step4.text,
                            image: Asset.overviewPage4.swiftUIImage
                        )
                    }
                }
                HStack {
                    Spacer()
                }
            }
            .padding(.top, Spacing.largeComponentSpacer)
            .edgesIgnoringSafeArea(.bottom)
            overlayView()
            if viewModel.currentSelectedPage == Constants.numberOfPages - 1 {
                VStack {
                    Spacer()
                    PrimaryButton(
                        action: viewModel.onContinueTap,
                        text: Localizable.OnboardingOverview.Action.continue.key,
                        style: .white()
                    )
                    .padding(.horizontal, Spacing.large)
                    .padding(.bottom, Spacing.large)
                }
            }
        }
    }

    @ViewBuilder
    func overlayView() -> some View {
        VStack(spacing: 0) {
            HStack {
                ForEach(0 ..< 4) { index in
                    RoundedRectangle(cornerRadius: CornerRadius.medium)
                        .frame(height: Heights.oviewviewPageIndicator)
                        .foregroundColor(
                            index <= self.viewModel.currentSelectedPage ? Asset.accentForegroundText
                                .swiftUIColor : Asset.fill30.swiftUIColor
                        )
                }
            }
            .padding(.bottom, Spacing.medium)
            HStack {
                Localizable.OnboardingOverview.Label.header.text
                    .foregroundColor(Asset.accentForegroundText.swiftUIColor.opacity(0.69))
                    .font(PrimaryFont.bodyM.font)
                    .padding(.vertical, Spacing.extraSmall)
                    .padding(.horizontal, Spacing.medium)
                    .background(Asset.fill12Light.swiftUIColor)
                    .clipShape(Capsule())
                Spacer()
                if viewModel.currentSelectedPage != Constants.numberOfPages - 1 {
                    Button(action: viewModel.onSkipTap) {
                        Localizable.OnboardingOverview.Action.skip.text
                            .foregroundColor(Asset.accentForegroundText.swiftUIColor)
                            .font(PrimaryFont.labelS.font)
                            .padding(Spacing.extraExtraSmall)
                    }
                }
            }
            Spacer()
        }
        .padding(.horizontal, Spacing.medium)
        .padding(.top, Spacing.medium)
    }
}

struct FeatureOverviewView: View {
    var text: Text
    var image: Image

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            text
                .font(SecondaryFont.bodyL.font)
                .foregroundColor(Asset.accentForegroundText.swiftUIColor)
                .lineSpacing(Spacing.extraExtraSmall)
            Spacer()
            HStack {
                image
            }
        }
        .padding(.horizontal, Spacing.large)
        .padding(.bottom, Spacing.large)
    }
}

extension OnboardingOverviewView {
    final class ViewModel: ObservableObject {
        private weak var stateMachine: OnboardingStateMachine!
        @Published var currentSelectedPage: Int = 0

        init(stateMachine: OnboardingStateMachine) {
            self.stateMachine = stateMachine
        }

        func onSkipTap() {
            stateMachine.onOverviewFinishTap()
        }

        func onContinueTap() {
            stateMachine.onOverviewFinishTap()
        }
    }
}

#if DEBUG
    struct OnboardingOverviewView_Previews: PreviewProvider {
        static var previews: some View {
            OnboardingOverviewView(viewModel: .init(stateMachine: OnboardingStateMachine()))
        }
    }
#endif

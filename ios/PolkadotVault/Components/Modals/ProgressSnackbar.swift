//
//  ProgressSnackbar.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 09/11/2022.
//

import SwiftUI

final class ProgressSnackbarViewModel: ObservableObject {
    @Published var title: String = ""
    @Published var cancelActionTitle: String = ""
    @Published var total: Int = 300
    @Published var current: Int = 120
    var cancelAction: () -> Void = {}
}

struct ProgressSnackbar: View {
    @ObservedObject var viewModel: ProgressSnackbarViewModel
    @Environment(\.safeAreaInsets) private var safeAreaInsets

    var body: some View {
        VStack {
            Spacer()
            VStack(spacing: Spacing.small) {
                HStack {
                    VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                        Text(viewModel.title)
                            .font(PrimaryFont.bodyL.font)
                            .foregroundColor(.accentForegroundText)
                        Text(
                            Localizable.Scanner.Label
                                .progress(min(viewModel.current, viewModel.total), viewModel.total)
                        )
                        .font(PrimaryFont.captionM.font)
                        .foregroundColor(.textAndIconsTertiarySolid)
                    }
                    Spacer()
                    Button(
                        action: { viewModel.cancelAction() }
                    ) {
                        Text(viewModel.cancelActionTitle)
                            .foregroundColor(.accentPink300)
                            .font(PrimaryFont.labelM.font)
                    }
                    .padding(.trailing, Spacing.extraSmall)
                }
                ProgressView(value: min(Float(viewModel.current) / Float(viewModel.total), 1))
                    .tint(.accentPink300)
                    .background(.fill18LightOnly)
                    .cornerRadius(2)
            }
            .padding(Spacing.medium)
            .frame(height: Heights.progressSnackbarHeight, alignment: .center)
            .background(.fill12Solid)
            .cornerRadius(CornerRadius.medium)
        }
        .padding(.horizontal, Spacing.medium)
        .padding(.bottom, Spacing.large + safeAreaInsets.bottom)
    }
}

extension View {
    /// Presents given `overlayView` over bottom edge with opacity transition. Dismiss view with bottom edge transition
    /// - Parameters:
    ///   - overlayView: view to be presented as overlay
    ///   - isPresented: action controller in form of `Bool`
    /// - Returns: view that modifier is applied to
    func bottomProgressSnackbar(
        _ viewModel: ProgressSnackbarViewModel,
        isPresented: Binding<Bool>
    ) -> some View {
        bottomEdgeOverlay(overlayView: ProgressSnackbar(viewModel: viewModel), isPresented: isPresented)
    }
}

struct ProgressSnackbarDemo: View {
    @ObservedObject var viewModel: ProgressSnackbarViewModel

    var body: some View {
        VStack {
            Text("Present info snackbar")
            Spacer()
        }
        .onAppear {
            viewModel.title = "Multipart data"
            viewModel.cancelActionTitle = "Cancel"
        }
        .bottomProgressSnackbar(
            viewModel,
            isPresented: Binding<Bool>.constant(true)
        )
    }
}

#if DEBUG
    struct ProgressSnackbar_Previews: PreviewProvider {
        static var previews: some View {
            ProgressSnackbarDemo(
                viewModel: ProgressSnackbarViewModel()
            )
            .preferredColorScheme(.light)
        }
    }

#endif

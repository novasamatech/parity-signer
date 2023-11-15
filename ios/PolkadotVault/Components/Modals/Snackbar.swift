//
//  Snackbar.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 08/09/2022.
//

import SwiftUI

final class BottomSnackbarPresentation: ObservableObject {
    @Published var viewModel: SnackbarViewModel = .init(title: "")
    @Published var isSnackbarPresented: Bool = false
}

struct SnackbarViewModel {
    let title: String
    let style: Snackbar.Style
    let tapToDismiss: Bool
    let countdown: CircularCountdownModel?

    init(
        title: String,
        style: Snackbar.Style = .info,
        tapToDismiss: Bool = true,
        countdown: CircularCountdownModel? = nil
    ) {
        self.title = title
        self.style = style
        self.tapToDismiss = tapToDismiss
        self.countdown = countdown
    }
}

struct Snackbar: View {
    private enum Constants {
        static let keyVisibilityTime: CGFloat = 60
    }

    enum Style {
        case info
        case warning

        var tintColor: Color {
            switch self {
            case .info:
                .fill12Solid
            case .warning:
                .accentRed400
            }
        }
    }

    private let viewModel: SnackbarViewModel

    init(
        viewModel: SnackbarViewModel
    ) {
        self.viewModel = viewModel
    }

    var body: some View {
        HStack {
            Text(viewModel.title)
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(.accentForegroundText)
                .padding(.horizontal, Spacing.medium)
                .lineSpacing(Spacing.extraSmall)
                .multilineTextAlignment(.leading)
                .fixedSize(horizontal: false, vertical: true)
            Spacer()
            if let countdown = viewModel.countdown {
                CircularProgressView(countdown)
                    .padding(.trailing, Spacing.medium)
            }
        }
        .padding(.vertical, Spacing.medium)
        .frame(
            minHeight: Heights.snackbarHeight,
            alignment: .center
        )
        .background(viewModel.style.tintColor)
        .cornerRadius(CornerRadius.small)
        .padding([.top, .bottom])
        .padding(.horizontal, Spacing.extraSmall)
    }
}

extension View {
    /// Presents given `overlayView` over bottom edge with opacity transition. Dismiss view with bottom edge transition
    /// - Parameters:
    ///   - overlayView: view to be presented as overlay
    ///   - isPresented: action controller in form of `Bool`
    /// - Returns: view that modifier is applied to
    func bottomSnackbar(
        _ viewModel: SnackbarViewModel,
        isPresented: Binding<Bool>,
        autodismissCounter: TimeInterval = 3
    ) -> some View {
        bottomEdgeOverlay(
            overlayView: Snackbar(viewModel: viewModel)
                .tapAndDelayDismiss(
                    autodismissCounter: autodismissCounter,
                    isTapToDismissActive: viewModel.tapToDismiss,
                    isPresented: isPresented
                ),
            isPresented: isPresented
        )
    }
}

struct SnackbarDemo: View {
    @State private var showInfo = false
    @State private var showWarning = false

    var body: some View {
        VStack {
            Text("Present info snackbar")
                .onTapGesture {
                    showInfo = true
                }
            Spacer()
        }.bottomSnackbar(
            SnackbarViewModel(
                title: "Metadata has been updated",
                style: .info,
                countdown: .init(counter: 60, viewModel: .snackbarCountdown, onCompletion: {})
            ),
            isPresented: $showInfo,
            autodismissCounter: 60
        )
    }
}

#if DEBUG
    struct Snackbar_Previews: PreviewProvider {
        @State private var showOverlay = false

        static var previews: some View {
            SnackbarDemo()
                .preferredColorScheme(.light)
                .previewDevice(PreviewDevice(rawValue: "iPod Touch (7th generation)"))
        }
    }
#endif

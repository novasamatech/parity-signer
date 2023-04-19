//
//  LogNoteModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 18/01/2023.
//

import SwiftUI

struct LogNoteModal: View {
    @StateObject var viewModel: ViewModel
    @FocusState var focused: Bool
    @State var animateBackground: Bool = false

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.onCancelTap()
            },
            animateBackground: $animateBackground,
            ignoredEdges: .top
        ) {
            VStack(spacing: Spacing.medium) {
                HStack {
                    Button(
                        action: viewModel.onCancelTap
                    ) {
                        Asset.xmarkButtonMedium.swiftUIImage
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            .frame(
                                width: Heights.navigationButton,
                                height: Heights.navigationButton,
                                alignment: .center
                            )
                    }
                    .padding(.leading, Spacing.extraExtraSmall)
                    Spacer()
                    CapsuleButton(
                        action: viewModel.onDoneTap,
                        title: Localizable.LogsList.Modal.AddNote.Action.done.string,
                        isDisabled: $viewModel.isActionDisabled
                    )
                }
                .padding(.top, -Spacing.extraSmall)
                .padding(.horizontal, Spacing.extraSmall)
                VStack(alignment: .leading, spacing: 0) {
                    Localizable.LogsList.Modal.AddNote.Label.title.text
                        .font(PrimaryFont.titleM.font)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .padding(.bottom, Spacing.medium)
                    Localizable.LogsList.Modal.AddNote.Label.header.text
                        .font(PrimaryFont.bodyM.font)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .padding(.bottom, Spacing.small)
                    TextEditor(text: $viewModel.note)
                        .frame(minHeight: Heights.minTextEditorHeight, maxHeight: Heights.maxTextEditorHeight)
                        .hiddenTextEditorBackground()
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyL.font)
                        .autocorrectionDisabled()
                        .autocapitalization(.none)
                        .keyboardType(.asciiCapable)
                        .submitLabel(.return)
                        .padding(.horizontal, Spacing.small)
                        .background(Asset.fill6.swiftUIColor)
                        .cornerRadius(CornerRadius.small)
                        .submitLabel(.done)
                        .focused($focused)
                }
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.small)
            }
            .background(Asset.backgroundTertiary.swiftUIColor)
        }
        .onAppear {
            focused = true
        }
    }
}

extension LogNoteModal {
    final class ViewModel: ObservableObject {
        @Binding var isPresented: Bool
        @Published var note: String = ""
        @Published var isActionDisabled: Bool = true
        private var cancelBag = CancelBag()
        private let logsService: LogsService
        private let snackBarPresentation: BottomSnackbarPresentation

        init(
            isPresented: Binding<Bool>,
            logsService: LogsService = LogsService(),
            snackBarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation
        ) {
            _isPresented = isPresented
            self.logsService = logsService
            self.snackBarPresentation = snackBarPresentation
            subscribeToUpdates()
        }

        func onCancelTap() {
            isPresented = false
        }

        func onDoneTap() {
            logsService.addCommentToLogs(note) { [weak self] result in
                guard let self else { return }
                switch result {
                case .success:
                    self.isPresented = false
                case let .failure(error):
                    self.snackBarPresentation.viewModel = .init(title: error.description)
                    self.snackBarPresentation.isSnackbarPresented = true
                }
            }
        }

        private func subscribeToUpdates() {
            $note.sink { newValue in
                self.isActionDisabled = newValue.isEmpty
            }
            .store(in: cancelBag)
        }
    }
}

#if DEBUG
    struct LogNoteModal_Previews: PreviewProvider {
        static var previews: some View {
            LogNoteModal(
                viewModel: .init(
                    isPresented: .constant(true)
                )
            )
            .preferredColorScheme(.light)
        }
    }
#endif

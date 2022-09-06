//
//  FullScreenRoundedModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/09/2022.
//

import SwiftUI

struct FullScreenRoundedModal<Content: View>: View {
    @Binding private var animateBackground: Bool
    private var backgroundTapAction: () -> Void
    private var content: () -> Content

    init(
        backgroundTapAction: @escaping () -> Void = {},
        animateBackground: Binding<Bool> = Binding<Bool>.constant(true),
        @ViewBuilder content: @escaping () -> Content
    ) {
        self.backgroundTapAction = backgroundTapAction
        _animateBackground = animateBackground
        self.content = content
    }

    var body: some View {
        ZStack(alignment: .bottom) {
            // Semi transparent background
            Spacer()
                .frame(idealHeight: .infinity)
                .background(animateBackground ? Color.black.opacity(0.5) : .clear)
                .onTapGesture(perform: backgroundTapAction)
                .onAppear {
                    withAnimation(
                        Animation.easeIn(duration: AnimationDuration.standard)
                            .delay(AnimationDuration.standard)
                    ) {
                        animateBackground.toggle()
                    }
                }
            // Modal content
            VStack(alignment: .leading, spacing: Spacing.medium, content: content)
                .padding([.bottom, .top], Spacing.medium)
                .padding([.leading, .trailing], 0)
                .background(Asset.backgroundSecondary.swiftUIColor)
                .cornerRadius(radius: CornerRadius.medium, corners: [.topLeft, .topRight])
        }
        .ignoresSafeArea()
        .transition(.move(edge: .leading))
    }
}
//
//struct FullScreenRoundedModal_Previews: PreviewProvider {
//    static var previews: some View {
//        VStack {
//            FullScreenRoundedModal {
//                Text("Test label")
//                    .padding()
//                Text("Test label")
//                    .padding()
//            }
//        }
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//        VStack {
//            FullScreenRoundedModal {
//                Text("Test label")
//                    .padding()
//                Text("Test label")
//                    .padding()
//            }
//        }
//        .preferredColorScheme(.light)
//        .previewLayout(.sizeThatFits)
//    }
//}

//
//  ContentView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI


struct MainScreenContainer: View {
    @EnvironmentObject var data: SignerDataModel
    @GestureState private var dragOffset = CGSize.zero
    var body: some View {
        if data.onboardingDone {
            if data.authenticated {
                VStack (spacing: 0) {
                Header()
                ZStack {
                    VStack (spacing:0) {
                        ScreenSelector(
                            screenData: data.actionResult.screenData,
                            pushButton: {action, details, seedPhrase in data.pushButton(action: action, details: details, seedPhrase: seedPhrase)},
                            getSeed: {seedName in return data.getSeed(seedName: seedName)},
                            doJailbreak: data.jailbreak,
                            pathCheck: {seed, path, network in
                                return substratePathCheck(seedName: seed, path: path, network: network, dbname: data.dbName)
                            },
                            createAddress: {path, seedName in data.createAddress(path: path, seedName: seedName)},
                            checkSeedCollision: {seedName in return data.checkSeedCollision(seedName: seedName)},
                            restoreSeed: {seedName, seedPhrase, createRoots in data.restoreSeed(seedName: seedName, seedPhrase: seedPhrase, createRoots: createRoots)}
                        )
                        Spacer()
                    }
                    ModalSelector()
                    AlertSelector()
                }
                .gesture(
                    DragGesture().updating($dragOffset, body: { (value, state, transaction) in
                        if value.startLocation.x < 20 && value.translation.width > 100 {
                            data.pushButton(action: .goBack)
                        }
                    })
                )
                //Certain places are better off without footer
                if data.actionResult.footer {
                    Footer()
                        .padding(.horizontal)
                        .padding(.vertical, 8)
                        .background(Color("Bg000"))
                }
            }
            .gesture(
                DragGesture().onEnded{drag in
                    if drag.translation.width < -20 {
                        data.pushButton(action: .goBack)
                    }
                }
            )
            .alert("Navigation error", isPresented: $data.parsingAlert, actions: {})
            } else {
                Button(action: {data.refreshSeeds()}) {
                    BigButton(
                        text: "Unlock app",
                        action: {
                            data.refreshSeeds()
                            data.totalRefresh()
                        }
                    )
                }
            }
        } else {
            if (data.protected) {
                if data.canaryDead /* || data.bsDetector.canaryDead)*/ {
                    Text("Please enable airplane mode, turn off bluetooth and wifi connection and disconnect all cables!").background(Color("Bg000"))
                } else {
                    LandingView(onboard: {data.onboard()})
                }
            } else {
                Text("Please protect device with pin or password!").background(Color("Bg000"))
            }
        }
    }
}

/*
 struct MainButtonScreen_Previews: PreviewProvider {
 static var previews: some View {
 MainScreenContainer()
 }
 }*/

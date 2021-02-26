// import { useNavigation } from '@react-navigation/native';
// import { StackNavigationProp } from '@react-navigation/stack';
// import testIDs from 'e2e/testIDs';
// import React, { useContext, useState } from 'react';
// import { ScrollView, StyleSheet, View } from 'react-native';
// import colors from 'styles/colors';
// import fontStyles from 'styles/fontStyles';
// import { Identity } from 'types/identityTypes';
// import { RootStackParamList } from 'types/routes';
// import { getIdentityName } from 'utils/identitiesUtils';
// import { navigateToLegacyAccountList, resetNavigationTo, resetNavigationWithNetworkChooser, unlockAndReturnSeed } from 'utils/navigationHelpers';

// import { AccountsContext } from '../context';
// import ButtonIcon from './ButtonIcon';
// import Separator from './Separator';
// import TransparentBackground from './TransparentBackground';

// function ButtonWithArrow(props: {
// 	onPress: () => void;
// 	testID?: string;
// 	title: string;
// }): React.ReactElement {
// 	return <ButtonIcon {...props}
// 		{...i_arrowOptions} />;
// }

// function IdentitiesSwitch(): React.ReactElement {
// 	const accountsStore = useContext(AccountsContext);
// 	const navigation: StackNavigationProp<RootStackParamList> = useNavigation();
// 	const [visible, setVisible] = useState(false);
// 	const { accounts, currentIdentity, identities } = accountsStore.state;
// 	// useEffect(() => {
// 	// 	const firstLogin: boolean = identities.length === 0;
// 	// 	if (currentIdentity === null && !firstLogin) {
// 	// 		setVisible(true);
// 	// 	}
// 	// }, [currentIdentity, identities]);

// 	const closeModalAndNavigate = <RouteName extends keyof RootStackParamList>(
// 		screenName: RouteName,
// 		params?: RootStackParamList[RouteName]
// 	): void => {
// 		setVisible(false);
// 		// @ts-ignore
// 		navigation.navigate(screenName, params);
// 	};

// 	const onIdentitySelectedAndNavigate = async <
// 		RouteName extends keyof RootStackParamList
// 	>(
// 		identity: Identity,
// 		screenName: RouteName,
// 		params?: RootStackParamList[RouteName]
// 	): Promise<void> => {
// 		await accountsStore.selectIdentity(identity);
// 		setVisible(false);

// 		if (screenName === 'Main') {
// 			resetNavigationTo(navigation, screenName, params);
// 		} else if (screenName === 'IdentityBackup') {
// 			const seedPhrase = await unlockAndReturnSeed(navigation);

// 			resetNavigationWithNetworkChooser(navigation, screenName, {
// 				isNew: false,
// 				seedPhrase
// 			});
// 		} else {
// 			resetNavigationWithNetworkChooser(navigation, screenName, params);
// 		}
// 	};

// 	const onLegacyListClicked = (): void => {
// 		setVisible(false);
// 		navigateToLegacyAccountList(navigation);
// 		accountsStore.resetCurrentIdentity();
// 	};

// 	const renderIdentityOptions = (identity: Identity): React.ReactElement => {
// 		return (
// 			<>
// 				<ButtonWithArrow
// 					onPress={(): Promise<void> =>
// 						// onIdentitySelectedAndNavigate(identity, 'IdentityManagement')
// 					}
// 					testID={testIDs.IdentitiesSwitch.manageIdentityButton}
// 					title="Manage Identity"
// 				/>
// 				<ButtonWithArrow
// 					onPress={(): Promise<void> =>
// 						onIdentitySelectedAndNavigate(identity, 'IdentityBackup')
// 					}
// 					title="Show Recovery Phrase"
// 				/>
// 			</>
// 		);
// 	};

// 	const renderCurrentIdentityCard = (): React.ReactNode => {
// 		if (!currentIdentity) return;

// 		const currentIdentityTitle = getIdentityName(currentIdentity, identities);

// 		return (
// 			<>
// 				<ButtonIcon
// 					iconName="user"
// 					iconSize={40}
// 					iconType="antdesign"
// 					onPress={(): Promise<void> =>
// 						onIdentitySelectedAndNavigate(currentIdentity, 'Main')
// 					}
// 					style={{ paddingLeft: 16 }}
// 					textStyle={fontStyles.h1}
// 					title={currentIdentityTitle}
// 				/>
// 				{renderIdentityOptions(currentIdentity)}
// 				<Separator style={{ marginBottom: 0 }} />
// 			</>
// 		);
// 	};

// 	const renderSettings = (): React.ReactElement => {
// 		return (
// 			<>
// 				<ButtonIcon
// 					iconName="info"
// 					iconSize={24}
// 					iconType="antdesign"
// 					onPress={(): void => closeModalAndNavigate('About')}
// 					style={styles.indentedButton}
// 					textStyle={fontStyles.t_big}
// 					title="About"
// 				/>
// 				<ButtonWithArrow
// 					onPress={(): void => closeModalAndNavigate('NetworkSettings')}
// 					title="Network Settings"
// 				/>
// 				<ButtonWithArrow
// 					onPress={(): void => closeModalAndNavigate('TermsAndConditions')}
// 					title="Terms and Conditions"
// 				/>
// 				<ButtonWithArrow
// 					onPress={(): void => closeModalAndNavigate('PrivacyPolicy')}
// 					title="Privacy Policy"
// 				/>
// 			</>
// 		);
// 	};

// 	const renderNonSelectedIdentity = (identity: Identity): React.ReactElement => {
// 		const title = getIdentityName(identity, identities);

// 		return (
// 			<ButtonIcon
// 				iconName="user"
// 				iconSize={24}
// 				iconType="antdesign"
// 				key={identity.encryptedSeed}
// 				onPress={(): Promise<void> =>
// 					onIdentitySelectedAndNavigate(identity, 'Main')
// 				}
// 				style={styles.indentedButton}
// 				textStyle={fontStyles.h2}
// 				title={title}
// 			/>
// 		);
// 	};

// 	const renderIdentities = (): React.ReactNode => {
// 		// if no identity or the only one we have is the selected one

// 		if (!identities.length || (identities.length === 1 && currentIdentity))
// 			return <Separator style={{ height: 0, marginVertical: 4 }} />;

// 		const identitiesToShow = currentIdentity
// 			? identities.filter(identity => identity.encryptedSeed !== currentIdentity.encryptedSeed)
// 			: identities;

// 		return (
// 			<>
// 				<ScrollView
// 					bounces={false}
// 					style={{ maxHeight: 160 }}
// 				>
// 					<View style={{ paddingVertical: 8 }}>
// 						{identitiesToShow.map(renderNonSelectedIdentity)}
// 					</View>
// 				</ScrollView>
// 				{identities.length > 5 && (
// 					<Separator shadow={true}
// 						style={{ marginTop: 0 }} />
// 				)}
// 			</>
// 		);
// 	};

// 	return (
// 		<View>
// 			<ButtonIcon
// 				iconBgStyle={{ backgroundColor: 'transparent' }}
// 				iconName="user"
// 				iconSize={26}
// 				iconType="antdesign"
// 				onPress={(): void => setVisible(!visible)}
// 				style={{ paddingHorizontal: 6 }}
// 				testID={testIDs.IdentitiesSwitch.toggleButton}
// 			/>

// 			<TransparentBackground
// 				animationType="none"
// 				setVisible={setVisible}
// 				style={styles.container}
// 				testID={testIDs.IdentitiesSwitch.modal}
// 				visible={visible}
// 			>
// 				<View style={styles.card}>
// 					{renderCurrentIdentityCard()}
// 					{renderIdentities()}
// 					{accounts.size > 0 && (
// 						<>
// 							<ButtonIcon
// 								iconName="solution1"
// 								iconSize={24}
// 								iconType="antdesign"
// 								onPress={onLegacyListClicked}
// 								style={styles.indentedButton}
// 								textStyle={fontStyles.t_big}
// 								title="Legacy Accounts"
// 							/>
// 							<Separator />
// 						</>
// 					)}

// 					<ButtonIcon
// 						iconName="plus"
// 						iconSize={24}
// 						iconType="antdesign"
// 						onPress={(): void => closeModalAndNavigate('RecoverAccount')}
// 						style={styles.indentedButton}
// 						testID={testIDs.IdentitiesSwitch.addIdentityButton}
// 						textStyle={fontStyles.t_big}
// 						title="Add Identity"
// 					/>

// 					<Separator />
// 					{__DEV__ && (
// 						<View>
// 							<ButtonIcon
// 								iconName="plus"
// 								iconSize={24}
// 								iconType="antdesign"
// 								onPress={(): void => closeModalAndNavigate('AccountNew')}
// 								style={styles.indentedButton}
// 								textStyle={fontStyles.t_big}
// 								title="Add legacy account"
// 							/>
// 							<Separator />
// 						</View>
// 					)}

// 					{renderSettings()}
// 				</View>
// 			</TransparentBackground>
// 		</View>
// 	);
// }

// const styles = StyleSheet.create({
// 	card: {
// 		backgroundColor: colors.background.app,
// 		borderRadius: 4,
// 		paddingBottom: 16,
// 		paddingTop: 8
// 	},
// 	container: {
// 		justifyContent: 'center',
// 		paddingHorizontal: 16
// 	},
// 	i_arrowStyle: {
// 		paddingLeft: 64,
// 		paddingTop: 0
// 	},
// 	indentedButton: { paddingLeft: 32 }
// });

// const i_arrowOptions = {
// 	iconColor: colors.signal.main,
// 	iconName: 'arrowright',
// 	iconSize: fontStyles.i_medium.fontSize,
// 	iconType: 'antdesign',
// 	style: styles.i_arrowStyle,
// 	textStyle: { ...fontStyles.a_text, color: colors.signal.main }
// };

// export default IdentitiesSwitch;

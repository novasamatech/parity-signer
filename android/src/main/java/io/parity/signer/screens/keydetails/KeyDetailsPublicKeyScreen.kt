package io.parity.signer.screens.keydetails

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Info
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.IdentIconWithNetwork
import io.parity.signer.components.NetworkLabelWithIcon
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.sharedcomponents.KeyCard
import io.parity.signer.components.sharedcomponents.ShowBase58Collapsible
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.domain.KeyDetailsModel
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.intoImageBitmap
import io.parity.signer.screens.scan.transaction.transactionElements.TCNameValueOppositeElement
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.encodeToQr
import kotlinx.coroutines.runBlocking

/**
 * Default main screen with list Seeds/root keys
 */
@Composable
fun KeyDetailsPublicKeyScreen(
	model: KeyDetailsModel,
	rootNavigator: Navigator,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeaderClose(
			stringResource(id = R.string.key_details_public_export_title),
			if (model.isRootKey) {
				null
			} else {
				stringResource(id = R.string.key_details_public_export_derived_subtitle)
			},
			onClose = { rootNavigator.backAction() },
			onMenu = { rootNavigator.navigate(Action.RIGHT_BUTTON_ACTION) }
		)
		Box(modifier = Modifier.weight(1f)) {
			Column(
				modifier = Modifier.verticalScroll(rememberScrollState())
			) {
				val plateShape =
					RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
				Column(
					modifier = Modifier
						.padding(start = 24.dp, end = 24.dp, top = 50.dp, bottom = 8.dp)
						.clip(plateShape)
						.border(
							BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
							plateShape
						)
						.background(MaterialTheme.colors.fill6, plateShape)
				) {
					Box(
						modifier = Modifier
							.fillMaxWidth(1f)
							.aspectRatio(1.1f)
							.background(
								Color.White,
								plateShape
							),
						contentAlignment = Alignment.Center,
					) {

						val isPreview = LocalInspectionMode.current
						val qrImage = remember {
							if (isPreview) {
								PreviewData.exampleQRImage
							} else {
								runBlocking { encodeToQr(model.qrData, false) }
							}
						}

						Image(
							bitmap = qrImage.intoImageBitmap(),
							contentDescription = stringResource(R.string.qr_with_address_to_scan_description),
							contentScale = ContentScale.Fit,
							modifier = Modifier.size(264.dp)
						)
					}
					Row(
						Modifier.padding(16.dp),
						verticalAlignment = Alignment.CenterVertically,
					) {
						ShowBase58Collapsible(
							base58 = model.address.cardBase.base58,
							modifier = Modifier
								.weight(1f)
								.padding(end = 24.dp)
						)
						IdentIconWithNetwork(
							identicon = model.address.cardBase.identIcon,
							networkLogoName = model.address.network,
							size = 36.dp,
							modifier = Modifier.padding(start = 24.dp)
						)
					}
				}
				Column(
					modifier = Modifier
						.padding(start = 24.dp, end = 24.dp, top = 8.dp, bottom = 8.dp)
						.background(MaterialTheme.colors.fill6, plateShape)
						.padding(horizontal = 16.dp)
				) {
					Row(
						horizontalArrangement = Arrangement.spacedBy(8.dp),
						modifier = Modifier.padding(vertical = 8.dp)
					) {
						Text(
							text = "Network",//todo dmitry fiinsh and export
							style = SignerTypeface.BodyL,
							color = MaterialTheme.colors.textTertiary
						)
						Spacer(modifier = Modifier.weight(1f))
						NetworkLabelWithIcon(
							model.networkInfo.networkTitle,
							model.networkInfo.networkLogo,
						)
					}
					SignerDivider(sidePadding = 0.dp)
					TCNameValueOppositeElement(
						name = "Derivation Path",
						value = model.address.cardBase.path,
						valueInSameLine = true,
					)
					SignerDivider(sidePadding = 0.dp)
					TCNameValueOppositeElement(
						name = "Key Set Name",
						value = model.address.cardBase.seedName,
						valueInSameLine = true,
					)
				}
				if (model.secretExposed) {
					ExposedKeyAlarm()
				}
			}
		}
	}
}

@Composable
fun KeyDetailsPublicKeyScreenold(
	//todo dmitry remove old
	model: KeyDetailsModel,
	rootNavigator: Navigator,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeaderClose(
			stringResource(id = R.string.key_details_public_export_title),
			if (model.isRootKey) {
				null
			} else {
				stringResource(id = R.string.key_details_public_export_derived_subtitle)
			},
			onClose = { rootNavigator.backAction() },
			onMenu = { rootNavigator.navigate(Action.RIGHT_BUTTON_ACTION) }
		)
		Box(modifier = Modifier.weight(1f)) {
			Column(
				modifier = Modifier.verticalScroll(rememberScrollState())
			) {
				val plateShape =
					RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
				Column(
					modifier = Modifier
						.padding(start = 24.dp, end = 24.dp, top = 50.dp, bottom = 8.dp)
						.clip(plateShape)
						.border(
							BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
							plateShape
						)
						.background(MaterialTheme.colors.fill6, plateShape)
				) {
					Box(
						modifier = Modifier
							.fillMaxWidth(1f)
							.aspectRatio(1.1f)
							.background(
								Color.White,
								plateShape
							),
						contentAlignment = Alignment.Center,
					) {

						val isPreview = LocalInspectionMode.current
						val qrImage = remember {
							if (isPreview) {
								PreviewData.exampleQRImage
							} else {
								runBlocking { encodeToQr(model.qrData, false) }
							}
						}

						Image(
							bitmap = qrImage.intoImageBitmap(),
							contentDescription = stringResource(R.string.qr_with_address_to_scan_description),
							contentScale = ContentScale.Fit,
							modifier = Modifier.size(264.dp)
						)
					}
					KeyCard(model.address)
				}
				if (model.secretExposed) {
					ExposedKeyAlarm()
				}
			}
		}
	}
}

@Composable
private fun ExposedKeyAlarm() {
	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	Row(
		modifier = Modifier
			.padding(vertical = 8.dp, horizontal = 24.dp)
			.border(
				BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
				innerShape
			)
			.background(MaterialTheme.colors.fill6, innerShape)

	) {
		Text(
			text = stringResource(R.string.key_details_exposed_notification_label),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyM,
			modifier = Modifier
				.weight(1f)
				.padding(start = 16.dp, top = 16.dp, bottom = 16.dp)
		)
		Icon(
			imageVector = Icons.Outlined.Info,
			contentDescription = null,
			tint = MaterialTheme.colors.red500,
			modifier = Modifier
				.align(Alignment.CenterVertically)
				.padding(start = 18.dp, end = 18.dp)
		)
	}
}


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewKeyDetailsScreenDerived() {
	val mockModel = KeyDetailsModel.createStubDerived()
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 700.dp)) {
			KeyDetailsPublicKeyScreen(mockModel, EmptyNavigator())
		}
	}
}

@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewKeyDetailsScreenRoot() {
	val mockModel = KeyDetailsModel.createStubRoot()
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 700.dp)) {
			KeyDetailsPublicKeyScreen(mockModel, EmptyNavigator())
		}
	}
}

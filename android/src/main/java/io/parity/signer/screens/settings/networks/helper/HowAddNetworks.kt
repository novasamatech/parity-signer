package io.parity.signer.screens.settings.networks.helper

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.*
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.screens.scan.errors.COMPOSE_URL_TAG_ANNOTATION
import io.parity.signer.ui.theme.*


//todo text export
@Composable
internal fun HowAddNetworks(
	onClose: Callback,
	onNext: Callback,
	onScanClicked: Callback,
) {
	Column() {
		ScreenHeaderWithButton(
			canProceed = true,
			btnText = "Next",
			onClose = onClose,
			onDone = onNext,
		)
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.padding(16.dp)
		) {
			Text(
				text = "Step 1/2",
				style = SignerTypeface.CaptionM,
				color = MaterialTheme.colors.textSecondary,
			)
			Text(
				text = "How to Add Networks",
				style = SignerTypeface.TitleL,
				color = MaterialTheme.colors.primary,
				modifier = Modifier.padding(bottom = 16.dp)
			)
			Row(
				modifier = Modifier
					.fillMaxWidth(1f)
					.background(
						MaterialTheme.colors.fill12,
						RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
					)
					.padding(16.dp)
			) {
				Box(
					modifier = Modifier
						.padding(end = 12.dp)
						.background(
							MaterialTheme.colors.backgroundSystem,
							CircleShape,
						)
						.defaultMinSize(32.dp, 32.dp),

					contentAlignment = Alignment.Center,
				) {
					Text(
						text = "1",
						style = SignerTypeface.BodyL,
						color = MaterialTheme.colors.primary,
					)
				}
				Column() {
					Text(
						text = "Go to Portal where \u2028your network is stored",
						style = SignerTypeface.BodyL,
						color = MaterialTheme.colors.primary,
						modifier = Modifier.padding(end = 12.dp)
					)
					Text(
						text = getDescriptionForLinks(),
						style = SignerTypeface.BodyM,
						color = MaterialTheme.colors.textSecondary,
						modifier = Modifier.padding(end = 12.dp)
					)
				}
			}

			Spacer(modifier = Modifier.padding(top = 4.dp))

			Row(
				modifier = Modifier
					.fillMaxWidth(1f)
					.background(
						MaterialTheme.colors.fill12,
						RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
					)
					.padding(16.dp)
			) {
				Box(
					modifier = Modifier
						.padding(end = 12.dp)
						.background(
							MaterialTheme.colors.backgroundSystem,
							CircleShape,
						)
						.defaultMinSize(32.dp, 32.dp),

					contentAlignment = Alignment.Center,
				) {
					Text(
						text = "2",
						style = SignerTypeface.BodyL,
						color = MaterialTheme.colors.primary,
					)
				}
				Column() {
					Text(
						text = "Choose the Network you need ",
						style = SignerTypeface.BodyL,
						color = MaterialTheme.colors.primary,
						modifier = Modifier.padding(end = 12.dp)
					)
				}
			}

			Spacer(modifier = Modifier.padding(top = 4.dp))

			Column(
				modifier = Modifier
					.fillMaxWidth(1f)
					.background(
						MaterialTheme.colors.fill12,
						RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
					)
					.padding(16.dp)) {
				Row() {
					Box(
						modifier = Modifier
							.padding(end = 12.dp)
							.background(
								MaterialTheme.colors.backgroundSystem,
								CircleShape,
							)
							.defaultMinSize(32.dp, 32.dp),

						contentAlignment = Alignment.Center,
					) {
						Text(
							text = "3",
							style = SignerTypeface.BodyL,
							color = MaterialTheme.colors.primary,
						)
					}
					Column() {
						Text(
							text = "Scan \"Chain spec\" QR to add the missing network",
							style = SignerTypeface.BodyL,
							color = MaterialTheme.colors.primary,
							modifier = Modifier.padding(end = 12.dp)
						)
					}
				}
				SecondaryButtonWide(
					label = "Scan Chain Spec",
					onClicked = onScanClicked,
					withBackground = true,
					modifier = Modifier.padding(top = 16.dp)
				)
			}

			Spacer(modifier = Modifier.padding(top = 4.dp))

			Row(
				modifier = Modifier
					.fillMaxWidth(1f)
					.background(
						MaterialTheme.colors.fill12,
						RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
					)
					.padding(16.dp)
			) {
				Box(
					modifier = Modifier
						.padding(end = 12.dp)
						.background(
							MaterialTheme.colors.backgroundSystem,
							CircleShape,
						)
						.defaultMinSize(32.dp, 32.dp),

					contentAlignment = Alignment.Center,
				) {
					Text(
						text = "4",
						style = SignerTypeface.BodyL,
						color = MaterialTheme.colors.primary,
					)
				}
				Column() {
					Text(
						text = "Press “Next” When You’re Done",
						style = SignerTypeface.BodyL,
						color = MaterialTheme.colors.primary,
						modifier = Modifier.padding(end = 12.dp)
					)
				}
			}
		}
	}
}

@Composable
@OptIn(ExperimentalTextApi::class)
internal fun getDescriptionForLinks(): AnnotatedString {
	val context = LocalContext.current
	return buildAnnotatedString {
		append("\n")
		withStyle(SpanStyle(color = MaterialTheme.colors.pink300)) {
			withAnnotation(
				COMPOSE_URL_TAG_ANNOTATION,
				"https://${context.getString(R.string.transaction_error_steps_2_url_core_networks)}"
			) {
				append(context.getString(R.string.transaction_error_steps_2_url_core_networks))
			}
		}
		append(stringResource(R.string.transaction_error_steps_2_core_networks_description))
		append("\n\n")
		withStyle(SpanStyle(color = MaterialTheme.colors.pink300)) {
			withAnnotation(
				COMPOSE_URL_TAG_ANNOTATION,
				"https://${context.getString(R.string.transaction_error_steps_3_url_parachains)}"
			) {
				append(context.getString(R.string.transaction_error_steps_3_url_parachains))
			}
		}
		append(stringResource(R.string.transaction_error_steps_3_description_parachains))
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
private fun PreviewHowAddNetworks() {
	SignerNewTheme {
		HowAddNetworks({}, {}, {})
	}
}

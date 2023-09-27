package io.parity.signer.screens.scan.errors

import android.content.Context
import android.content.res.Configuration
import android.widget.Toast
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.*
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.*

/**
 * Bottom sheet created to show Transaction errors in scan flow,
 * but used to show errors in a bottoms sheets in other places as well
 */
@Composable
fun LocalErrorBottomSheet(
	error: LocalErrorSheetModel,
	onOk: Callback
) {
	Column(
		Modifier
			.fillMaxWidth(1f)
			.verticalScroll(rememberScrollState())
	) {
		Text(
			text = error.title,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleM,
			modifier = Modifier.padding(
				top = 32.dp,
				bottom = 8.dp,
				start = 32.dp,
				end = 32.dp
			),
		)
		Text(
			text = error.subtitle,
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyM,
			modifier = Modifier
				.padding(horizontal = 32.dp)
				.padding(bottom = 16.dp),
		)

		if (error.showNetworkSteps) {
			val descriptionSteps = getDescriptionForUpdateMetadata()
			Column(
				modifier = Modifier
					.fillMaxWidth(1f)
					.padding(horizontal = 24.dp)
					.background(
						MaterialTheme.colors.fill6,
						RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
					)
					.border(
						1.dp,
						MaterialTheme.colors.fill12,
						RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
					)
			) {
				descriptionSteps.forEachIndexed() { index, step ->
					Row(
						modifier = Modifier.padding(vertical = 12.dp, horizontal = 16.dp)
					) {
						Text(
							text = (index + 1).toString(),
							color = MaterialTheme.colors.textTertiary,
							style = SignerTypeface.BodyL,
							modifier = Modifier.padding(end = 16.dp)
						)

						val context = LocalContext.current
						val layoutResult = remember {
							mutableStateOf<TextLayoutResult?>(null)
						}

						Text(
							text = step,
							color = MaterialTheme.colors.primary,
							style = SignerTypeface.BodyL,
							modifier = Modifier.pointerInput(Unit) {
								detectTapGestures { offsetPosition ->

									layoutResult.value?.let {
										val position = it.getOffsetForPosition(offsetPosition)
										step.getStringAnnotations(
											COMPOSE_URL_TAG_ANNOTATION,
											position,
											position
										)
											.firstOrNull()
											?.let { result ->
												Toast.makeText(
													context,
													context.getString(R.string.general_error_link_clicked_on_airgap_device),
													Toast.LENGTH_LONG
												).show()
												//this device is airgapped, don't open links!
//												uriHandler.openUri(result.item)
											}
									}
								}
							},
							onTextLayout = { layoutResult.value = it }
						)
					}
				}
			}
		}
		if (error.description != null) {
			Column(
				modifier = Modifier
					.fillMaxWidth(1f)
					.padding(horizontal = 24.dp)
					.background(
						MaterialTheme.colors.fill6,
						RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
					)
					.border(
						1.dp,
						MaterialTheme.colors.fill12,
						RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
					)
			) {
				Text(
					text = error.description,
					color = MaterialTheme.colors.primary,
					style = SignerTypeface.BodyL,
					modifier = Modifier.padding(16.dp)
				)
			}
		}
		SecondaryButtonWide(
			label = stringResource(id = R.string.generic_ok),
			modifier = Modifier.padding(24.dp),
			withBackground = true,
			onClicked = onOk,
		)
	}
}

@Composable
@OptIn(ExperimentalTextApi::class)
private fun getDescriptionForUpdateMetadata(): List<AnnotatedString> {
	val context = LocalContext.current
	val firstStringElement = buildAnnotatedString {
		append(stringResource(R.string.transaction_error_steps_1))
		append("\n\n")
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
		append("\n\n")
		withStyle(SpanStyle(color = MaterialTheme.colors.textTertiary)) {
			append(stringResource(R.string.transaction_error_steps_4_notes_for_other_networks))
		}
	}
	return listOf<AnnotatedString>(
		firstStringElement,
		AnnotatedString(stringResource(R.string.transaction_error_steps_choose_network)),
		AnnotatedString(stringResource(R.string.transaction_error_steps_scan_qr_code))
	)
}

data class LocalErrorSheetModel(
	val title: String,
	val subtitle: String,
	val description: String? = null,
	val showNetworkSteps: Boolean = false,
) {
	constructor(
		title: String,
		subtitle: String,
		details: String? = null,
	) : this(
		title = title,
		subtitle = subtitle,
		description = details
	)

	constructor(
		context: Context,
		details: String,
	) : this(
		title = context.getString(R.string.transaction_generic_error_title),
		subtitle = context.getString(R.string.transaction_generic_error_description),
		description = details
	)
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
private fun PreviewTransactionErrorBottomSheet() {
	SignerNewTheme {
		val model =
			TransactionError.MetadataForUnknownNetwork("Westend").toBottomSheetModel()
		LocalErrorBottomSheet(
			error = model, onOk = {}
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
private fun PreviewTransactionErrorBottomSheetCustom() {
	SignerNewTheme {
		val context = LocalContext.current
		val model =
			LocalErrorSheetModel(
				context = context,
				details = "Bad input data. Metadata for westend9330 is already in the database."
			)
		LocalErrorBottomSheet(
			error = model, onOk = {}
		)
	}
}

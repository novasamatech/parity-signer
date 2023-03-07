package io.parity.signer.screens.scan.errors

import android.content.res.Configuration
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
import androidx.compose.ui.platform.LocalUriHandler
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.TextLayoutResult
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.*


/**
 * todo dmitry similar to ios/PolkadotVault/Modals/Errors/ErrorBottomModalViewModel.swift:10
 *
 * todo dmitry ios/PolkadotVault/Core/Adapters/BackendNavigationAdapter.swift:48
 */


@Composable
fun TransactionErrorBottomSheet(
	model: TransactionErrorModel,
	onOk: Callback
) {
	Column(
		Modifier
			.fillMaxWidth(1f)
			.verticalScroll(rememberScrollState())
	) {
		Text(
			text = model.title,
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
			text = model.subtitle,
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyM,
			modifier = Modifier
				.padding(horizontal = 32.dp)
				.padding(bottom = 16.dp),
		)

		if (model.descriptionSteps.isNotEmpty()) {
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
				model.descriptionSteps.forEachIndexed() { index, step ->
					Row(
						modifier = Modifier.padding(vertical = 12.dp, horizontal = 16.dp)
					) {
						if (model.descriptionSteps.size > 1) {
							Text(
								text = (index + 1).toString(),
								color = MaterialTheme.colors.textTertiary,
								style = SignerTypeface.BodyL,
								modifier = Modifier.padding(end = 16.dp)
							)
						}

						val uriHandler = LocalUriHandler.current
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
												uriHandler.openUri(result.item)
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
		SecondaryButtonWide(
			label = stringResource(id = R.string.generic_ok),
			modifier = Modifier.padding(24.dp),
			withBackground = true,
			onClicked = onOk,
		)
	}
}

data class TransactionErrorModel(
	val title: String,
	val subtitle: String,
	val descriptionSteps: List<AnnotatedString> = emptyList()
)


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
		TransactionErrorBottomSheet(
			model = model, onOk = {}
		)
	}
}

package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Divider
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.*

@Composable
fun PrimaryButtonBottomSheet(
	label: String,
	modifier: Modifier = Modifier,
	isEnabled: Boolean = true,
	onClicked: Callback,
) {
	PrimaryButtonNotWide(
		label, modifier.fillMaxWidth(1f),
		isEnabled, onClicked
	)
}

@Composable
fun PrimaryButtonNotWide(
	label: String,
	modifier: Modifier = Modifier,
	isEnabled: Boolean = true,
	onClicked: Callback
) {
	Column(
		modifier = modifier
			.run {
				if (isEnabled) {
					clickable(onClick = onClicked)
				} else {
					this
				}
			}
			.background(
				if (isEnabled) {
					MaterialTheme.colors.pink500
				} else {
					MaterialTheme.colors.fill6
				},
				RoundedCornerShape(dimensionResource(id = R.dimen.buttonCornerRadius))
			)
			.padding(vertical = dimensionResource(id = R.dimen.buttonVerticalPadding)),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		Text(
			text = label,
			color = if (isEnabled) {
				Color.White
			} else {
				MaterialTheme.colors.textDisabled
			},
			style = SignerTypeface.TitleS,
			maxLines = 1,
		)
	}
}

@Composable
fun SecondaryButtonBottomSheet(
	label: String,
	modifier: Modifier = Modifier,
	withBackground: Boolean = false,
	onClicked: Callback,
) {
	Column(
		modifier = modifier
			.clickable(onClick = onClicked)
			.run {
				if (withBackground) {
					background(
						MaterialTheme.colors.fill18,
						RoundedCornerShape(dimensionResource(id = R.dimen.buttonCornerRadius)),
					)
				} else {
					this
				}
			}
			.padding(vertical = dimensionResource(id = R.dimen.buttonVerticalPadding))
			.fillMaxWidth(),
		horizontalAlignment = Alignment.CenterHorizontally
	) {
		Text(
			text = label,
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.TitleS,
			maxLines = 1,
		)
	}
}

@Composable
fun RowButtonsBottomSheet(
	labelCancel: String,
	labelCta: String,
	onClickedCancel: Callback,
	onClickedCta: Callback,
) {
	Row {
		Column(
			modifier = Modifier
				.clickable(onClick = onClickedCancel)
				.background(
					MaterialTheme.colors.fill18, RoundedCornerShape(
						dimensionResource(id = R.dimen.buttonCornerRadius)
					)
				)
				.padding(vertical = dimensionResource(id = R.dimen.buttonVerticalPadding))
				.weight(1f),
			horizontalAlignment = Alignment.CenterHorizontally
		) {
			Text(
				text = labelCancel,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
				maxLines = 1,
			)
		}

		Spacer(modifier = Modifier.padding(end = 8.dp))
		Column(
			modifier = Modifier
				.clickable(onClick = onClickedCta)
				.background(
					MaterialTheme.colors.pink500, RoundedCornerShape(
						dimensionResource(id = R.dimen.buttonCornerRadius)
					)
				)
				.padding(vertical = dimensionResource(id = R.dimen.buttonVerticalPadding))
				.weight(1f),
			horizontalAlignment = Alignment.CenterHorizontally
		) {
			Text(
				text = labelCta,
				color = Color.White,
				style = SignerTypeface.TitleS,
				maxLines = 1,
			)
		}
	}
}

@Composable
fun SignerBottomSheetDivider(
	modifier: Modifier = Modifier,
	padding: Dp = 16.dp,
) {
	Divider(
		color = MaterialTheme.colors.appliedSeparator,
		thickness = 1.dp,
		modifier = modifier.padding(horizontal = padding),
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
private fun PreviewCtaButtons() {
	SignerNewTheme {
		Column(
			modifier = Modifier.size(300.dp),
		) {
			PrimaryButtonBottomSheet("button") {}
			RowButtonsBottomSheet(
				labelCancel = "Cancel",
				labelCta = "Apply",
				onClickedCancel = {},
				onClickedCta = {},
			)
			SecondaryButtonBottomSheet("Secondary Bottom Sheet") {}
			SecondaryButtonBottomSheet(
				"Secondary with background",
				withBackground = true
			) {}
			PrimaryButtonNotWide(label = "primary slim") {}
			SignerBottomSheetDivider()
		}
	}
}

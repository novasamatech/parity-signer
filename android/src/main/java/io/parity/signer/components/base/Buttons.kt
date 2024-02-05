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
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.*

@Composable
fun PrimaryButtonWide(
	label: String,
	modifier: Modifier = Modifier,
	isEnabled: Boolean = true,
	activeText: Color = Color.White,
	activeBackground: Color = MaterialTheme.colors.pink500,
	onClicked: Callback,
) {
	PrimaryButton(
		label = label,
		modifier = modifier.fillMaxWidth(1f),
		isEnabled = isEnabled,
		activeText = activeText,
		activeBackground = activeBackground,
		onClicked = onClicked,
	)
}

@Composable
fun PrimaryButton(
	label: String,
	modifier: Modifier = Modifier,
	isEnabled: Boolean = true,
	activeText: Color = Color.White,
	activeBackground: Color = MaterialTheme.colors.pink500,
	onClicked: Callback
) {
	Box(
		modifier = modifier
			.clip(RoundedCornerShape(dimensionResource(id = R.dimen.buttonCornerRadius)))
			.run {
				if (isEnabled) {
					clickable(onClick = onClicked)
				} else {
					this
				}
			}
			.background(
				if (isEnabled) activeBackground else MaterialTheme.colors.primaryButtonDisabledBackground,
				RoundedCornerShape(dimensionResource(id = R.dimen.buttonCornerRadius))
			),
		contentAlignment = Alignment.Center,
	) {
		Text(
			text = label,
			color = if (isEnabled) activeText else MaterialTheme.colors.primaryButtonDisabledText,
			style = SignerTypeface.TitleS,
			textAlign = TextAlign.Center,
			maxLines = 1,
			modifier = Modifier
				.padding(
					horizontal = dimensionResource(id = R.dimen.buttonHorizontalTextPadding),
					vertical = dimensionResource(id = R.dimen.buttonVerticalPadding)
				)
		)
	}
}

@Composable
fun PrimaryButtonGreyDisabled(
	label: String,
	modifier: Modifier = Modifier,
	isEnabled: Boolean = true,
	onClicked: Callback
) {
	Column(
		modifier = modifier
			.clip(RoundedCornerShape(dimensionResource(id = R.dimen.buttonCornerRadius)))
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
			.padding(vertical = 12.dp), // in other places 16, but this is exception to show in header
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
			modifier = Modifier.padding(horizontal = dimensionResource(id = R.dimen.buttonHorizontalTextPadding))
		)
	}
}

@Composable
fun SecondaryButtonWide(
	label: String,
	modifier: Modifier = Modifier,
	isEnabled: Boolean = true,
	withBackground: Boolean = false,
	textColor: Color = MaterialTheme.colors.textSecondary,
	onClicked: Callback,
) {
	SecondaryButton(
		label = label,
		modifier = modifier.fillMaxWidth(),
		isEnabled  = isEnabled,
		withBackground = withBackground,
		textColor = textColor,
		onClicked = onClicked,
	)
}

@Composable
private fun SecondaryButton(
	label: String,
	modifier: Modifier = Modifier,
	isEnabled: Boolean = true,
	withBackground: Boolean = false,
	textColor: Color = MaterialTheme.colors.textSecondary,
	onClicked: Callback,
) {
	Column(
		modifier = modifier
			.clip(RoundedCornerShape(dimensionResource(id = R.dimen.buttonCornerRadius)))
			.run {
				if (isEnabled) {
					clickable(onClick = onClicked)
				} else {
					this
				}
			}
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
			.padding(vertical = dimensionResource(id = R.dimen.buttonVerticalPadding)),
		horizontalAlignment = Alignment.CenterHorizontally
	) {
		Text(
			text = label,
			color = textColor,
			style = SignerTypeface.TitleS,
			maxLines = 1,
		)
	}
}

@Composable
fun RowButtonsBottomSheet(
	labelCancel: String,
	labelCta: String,
	modifier: Modifier = Modifier,
	onClickedCancel: Callback,
	onClickedCta: Callback,
	isCtaDangerous: Boolean = false,
	isCtaEnabled: Boolean = true,
) {
	Row(modifier = modifier) {
		SecondaryButton(
			label = labelCancel,
			modifier = Modifier.weight(1f),
			withBackground = true,
			onClicked = onClickedCancel,
		)
		Spacer(modifier = Modifier.padding(end = 8.dp))
		PrimaryButton(
			label = labelCta, modifier = Modifier.weight(1f),
			activeBackground = if (isCtaDangerous) MaterialTheme.colors.red400
			else MaterialTheme.colors.pink500,
			onClicked = onClickedCta, isEnabled = isCtaEnabled
		)
	}
}

@Composable
fun SignerDivider(
	modifier: Modifier = Modifier,
	sidePadding: Dp = 16.dp,
) {
	Divider(
		color = MaterialTheme.colors.appliedSeparator,
		thickness = 1.dp,
		modifier = modifier.padding(horizontal = sidePadding),
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
private fun PreviewButtons() {
	SignerNewTheme {
		Column() {
			PrimaryButtonWide("button") {}
			RowButtonsBottomSheet(
				labelCancel = "Cancel",
				labelCta = "Apply",
				onClickedCancel = {},
				onClickedCta = {},
			)
			RowButtonsBottomSheet(
				labelCancel = "Cancel",
				labelCta = "Apply",
				onClickedCancel = {},
				onClickedCta = {},
				isCtaEnabled = false,
			)
			PrimaryButtonGreyDisabled(label = "grey enabled") {
			}
			PrimaryButtonGreyDisabled(
				label = "grey disabled",
				isEnabled = false,
			) {
			}
			PrimaryButton(label = "Primary enabled") {
			}
			PrimaryButton(
				label = "Primary disabled",
				isEnabled = false,
			) {
			}
			SecondaryButtonWide("Secondary Bottom Sheet") {}
			SecondaryButtonWide(
				"Secondary with background",
				withBackground = true
			) {}
			SignerDivider()
		}
	}
}

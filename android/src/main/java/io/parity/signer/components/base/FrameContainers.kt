package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.annotation.StringRes
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.HelpOutline
import androidx.compose.material.icons.outlined.Info
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.domain.conditional
import io.parity.signer.ui.theme.*


@Composable
fun NotificationFrameText(
	message: String,
	modifier: Modifier = Modifier,
	withBorder: Boolean = true,
) {
	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	Row(
		modifier = modifier
			.padding(8.dp)
			.conditional(withBorder) {
				border(
					BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
					innerShape,
				)
			}
			.background(MaterialTheme.colors.fill6, innerShape)
	) {
		Text(
			text = message,
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.CaptionM,
			modifier = Modifier
				.weight(1f)
				.padding(start = 16.dp, top = 16.dp, bottom = 16.dp)
		)
		Icon(
			imageVector = Icons.Outlined.Info,
			contentDescription = null,
			tint = MaterialTheme.colors.pink300,
			modifier = Modifier
				.align(Alignment.CenterVertically)
				.padding(start = 18.dp, end = 18.dp)
		)
	}
}

@Composable
fun NotificationFrameTextImportant(
	message: String,
	withBorder: Boolean = true,
	textColor: Color = MaterialTheme.colors.pink300,
	modifier: Modifier = Modifier,
) {
	val BACKGROUND = Color(0x14F272B6)
	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	Row(
		modifier = modifier
			.conditional(withBorder) {
				border(
					BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
					innerShape
				)
			}
			.background(BACKGROUND, innerShape)

	) {
		Text(
			text = message,
			color = textColor,
			style = SignerTypeface.CaptionM,
			modifier = Modifier
				.weight(1f)
				.padding(start = 16.dp, top = 16.dp, bottom = 16.dp)
		)
		Icon(
			imageVector = Icons.Outlined.Info,
			contentDescription = null,
			tint = MaterialTheme.colors.pink300,
			modifier = Modifier
				.align(Alignment.CenterVertically)
				.padding(start = 18.dp, end = 18.dp)
		)
	}
}


@Composable
fun NotificationFrameTextAlert(
	message: String,
	modifier: Modifier = Modifier,
) {
	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	Row(
		modifier = modifier
			.border(
				BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
				innerShape
			)
	) {
		Text(
			text = message,
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyM,
			modifier = Modifier
				.weight(1f)
				.padding(start = 16.dp, top = 16.dp, bottom = 16.dp)
		)
		Icon(
			imageVector = Icons.Outlined.Info,
			contentDescription = null,
			tint = MaterialTheme.colors.pink300,
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
private fun PreviewFrameContainers() {
	SignerNewTheme {
		Column(
			modifier = Modifier,
		) {
			NotificationFrameText(message = stringResource(id = R.string.key_set_export_description_content))
			SignerDivider()
			NotificationFrameTextImportant(
				message = stringResource(id = R.string.key_set_export_description_content),
				modifier = Modifier.padding(horizontal = 8.dp, vertical = 10.dp),
			)
			SignerDivider()
			NotificationFrameTextImportant(
				message = stringResource(id = R.string.key_set_export_description_content),
				modifier = Modifier.padding(horizontal = 8.dp, vertical = 10.dp),
				withBorder = false
			)
			SignerDivider()
			NotificationFrameTextAlert(
				message = stringResource(id = R.string.key_set_export_description_content),
				modifier = Modifier.padding(horizontal = 8.dp, vertical = 10.dp),
			)
		}
	}
}

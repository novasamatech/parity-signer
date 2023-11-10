package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.zIndex
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.pink500


@Composable
fun ScanIconComponent(
	onClick: Callback,
	modifier: Modifier,
) {
	ScanIcon(
		onClick,
		modifier
			.zIndex(1f)
			.shadow(
				elevation = 16.dp,
				shape = CircleShape,
			)
	)
}


@Composable
fun ScanIcon(
	onClick: Callback,
	modifier: Modifier = Modifier,
) {
	Box(
		modifier = modifier
			.size(72.dp)
			.clickable(onClick = onClick)
			.border(6.dp, MaterialTheme.colors.primary, CircleShape)
			.background(MaterialTheme.colors.background),
		contentAlignment = Alignment.Center,
	) {
		Box(
			modifier = Modifier
				.background(MaterialTheme.colors.pink500, CircleShape)
				.size(56.dp)
		)
		Image(
			painter = painterResource(R.drawable.ic_qr_code_2),
			contentDescription = stringResource(R.string.description_scan_icon),
			colorFilter = ColorFilter.tint(Color.White),
			modifier = Modifier.size(32.dp)
		)
	}
}



@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewScanIcon() {
	SignerNewTheme {
		Column() {
			ScanIcon(onClick = {})
		}
	}
}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Composable
private fun PreviewScanIconComponent() {
	SignerNewTheme {
		Column() {
			Box(
				modifier = Modifier.size(90.dp),
				contentAlignment = Alignment.Center,
			) {
				ScanIconComponent(onClick = {}, modifier = Modifier)
			}
		}
	}
}


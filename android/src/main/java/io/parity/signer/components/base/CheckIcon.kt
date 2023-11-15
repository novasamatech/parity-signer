package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.fill18

@Composable
fun CheckIcon(
	modifier: Modifier = Modifier,
	size: Dp = 28.dp,
	noBackground: Boolean = false,
) {
	Box(
		modifier = modifier
			.size(size)
			.run {
				if (noBackground) {
					this
				} else {
					background(
						MaterialTheme.colors.fill18,
						CircleShape
					)
				}
			},
		contentAlignment = Alignment.Center,
	) {
		Image(
			imageVector = Icons.Default.Check,
			contentDescription = stringResource(R.string.description_check_botton),
			colorFilter = ColorFilter.tint(MaterialTheme.colors.primary),
			modifier = Modifier
				.size(20.dp)
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
private fun PreviewCheckIcon() {
	SignerNewTheme {
		Column() {
			CheckIcon(noBackground = true)
			CheckIcon(noBackground = false)
		}
	}
}

package io.parity.signer.components.networkicon

import android.annotation.SuppressLint
import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.AutoSizeText
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun NetworkIcon(
	networkLogoName: String,
	modifier: Modifier = Modifier,
	size: Dp = 32.dp,
) {
	val icon = getIconForNetwork(networkLogoName)
	if (icon != null) {
		Image(
			painter = icon,
			contentDescription = null,
			modifier = modifier
				.clip(CircleShape)
				.size(size),
		)
	} else {
		val networkColors = ServiceLocator.unknownNetworkColorsGenerator
			.getBackground(networkLogoName)
			.toUnknownNetworkColorsDrawable()
		val chars = networkLogoName.take(2).uppercase()
		UnknownNetworkIcon(networkColors, chars, size, modifier)
	}
}

@Composable
private fun UnknownNetworkIcon(
	networkColors: UnknownNetworkColorDrawable,
	chars: String,
	size: Dp,
	modifier: Modifier = Modifier
) {
	Box(
		modifier = modifier
			.size(size)
			.padding(size/6)
			.background(networkColors.background, CircleShape),
		contentAlignment = Alignment.Center
	) {
		AutoSizeText(
			text = chars,
			color = networkColors.text,
		)
	}
}

@Composable
@SuppressLint("DiscouragedApi")
private fun getIconForNetwork(networkName: String): Painter? {
//	val resource = resources.getIdentifier(/* name = */ "network_$networkName",
//		/* defType = */"drawable",/* defPackage = */packageName)

	val id = when (networkName) {
		"polkadot" -> R.drawable.network_polkadot
		else -> -1
	}

	return if (id > 0) {
		painterResource(id = id)
	} else {
		null
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
private fun PreviewNetworkIconSizes() {
	SignerNewTheme {
		Column {
			NetworkIcon("polkadot")
			NetworkIcon("some_unknown")
			NetworkIcon("polkadot", size = 16.dp)
			NetworkIcon("some_unknown2", size = 16.dp)
			NetworkIcon("polkadot", size = 56.dp)
			NetworkIcon("some_unknown3", size = 56.dp)
		}
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
private fun PreviewNetworkIconUnknownIcons() {
	SignerNewTheme {
		Column {
			val colors = UnknownNetworkColors.values()
			colors.forEach { color ->
				UnknownNetworkIcon(
					networkColors = color.toUnknownNetworkColorsDrawable(),
					chars = "SO",
					size = 24.dp
				)
			}
		}
	}
}

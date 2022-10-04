package io.parity.signer.components.items

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.IdentIcon
import io.parity.signer.screens.KeySetViewModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*

//todo dmitry finish item
@Composable
fun KeySetItem(
	model: KeySetViewModel,
) {
	Surface(
		shape = MaterialTheme.shapes.medium,
		color = MaterialTheme.colors.Bg200,
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically,
		) {
				IdentIcon(identicon = model.identicon, size = 36.dp)
				Column() {
					Text(
						text = model.seedName,
						color = MaterialTheme.colors.primary,
						style = TypefaceNew.LabelM,
					)
					Text(
						text = model.seedName,
						color = MaterialTheme.colors.textDisabled,
						style = TypefaceNew.BodyM,
					)
				}
			Image(
				imageVector = Icons.Filled.ChevronRight,
				contentDescription = null,
				colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
				modifier = Modifier
					.size(28.dp)
			)
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
private fun PreviewKeySetItem() {
	SignerNewTheme {
		KeySetItem(
			KeySetViewModel("My special key set", PreviewData.exampleIdenticon, 2.toUInt())
		)
	}
}

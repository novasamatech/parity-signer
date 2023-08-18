package io.parity.signer.components.networkicon.jdenticon

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun Jdenticon(
	seed: String,
	size: Dp,
	modifier: Modifier = Modifier
) {

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
private fun PreviewJdenticon() {
	SignerNewTheme {
		val seed_name = "8PegJD6VsjWwinrP6AfgNqejWYdJ8KqF4xutpyq7AdFJ3W5"
		Column(horizontalAlignment = Alignment.CenterHorizontally) {
			Jdenticon(seed_name, 48.dp)
			Jdenticon(seed_name, 32.dp)
		}
	}
}

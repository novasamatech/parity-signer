import android.content.res.Configuration
import androidx.compose.animation.Crossfade
import androidx.compose.animation.core.tween
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.size
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textTertiary

/**
 * Our custom ios style checkbox
 */
@Composable
fun SignerCheckbox(
	isChecked: Boolean,
	modifier: Modifier = Modifier,
	checkedColor: Color = MaterialTheme.colors.pink300,
	uncheckedColor: Color = MaterialTheme.colors.textTertiary,
	onClicked: Callback,
) {
	Box(
		modifier = modifier
			.size(32.dp)
			.clickable(onClick = onClicked),
		contentAlignment = Alignment.Center,
	) {
		Crossfade(
			targetState = isChecked,
			animationSpec = tween(durationMillis = 150),
		) { isChecked ->
			Image(
				painter = painterResource(
					id = if (isChecked)
						R.drawable.circle_checked_24 else R.drawable.circle_unckecked_24
				),
				contentDescription = stringResource(R.string.description_checkbox),
				colorFilter = ColorFilter.tint(if (isChecked) checkedColor else uncheckedColor),
				modifier = Modifier
					.size(24.dp)
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
private fun PreviewSignerCheckbox() {
	SignerNewTheme {
		Column() {
			SignerCheckbox(isChecked = true) {
			}
			SignerCheckbox(isChecked = false) {
			}
		}
	}
}

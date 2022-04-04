package io.parity.signer.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.gestures.rememberDraggableState
import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.IconButton
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircleOutline
import androidx.compose.material.icons.outlined.ExpandCircleDown
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.ShieldAlert
import io.parity.signer.components.BottomMultiselectBar
import io.parity.signer.components.KeySelector
import io.parity.signer.components.NetworkLogoName
import io.parity.signer.components.SeedCard
import io.parity.signer.ui.theme.Action400
import io.parity.signer.ui.theme.Bg100
import io.parity.signer.ui.theme.Bg200
import org.json.JSONArray
import org.json.JSONObject
import kotlin.math.absoluteValue

/**
 * Key manager screen.
 */
@Composable
fun KeyManager(
	button: (button: ButtonID, details: String) -> Unit,
	increment: (Int, String) -> Unit,
	screenData: JSONObject,
	alertState: ShieldAlert?
) {
	val rootKey = screenData.optJSONObject("root") ?: JSONObject()
	val keySet = screenData.optJSONArray("set") ?: JSONArray()
	//val network = signerDataModel.screenData.value?.optJSONObject("network")
	val multiselectMode = screenData.optBoolean("multiselect_mode")
	val multiselectCount = screenData.optString("multiselect_count")
	var offsetX by remember { mutableStateOf(0f) }

	Box {
		Column {
			Row(
				Modifier
					.pointerInput(Unit) {
						detectTapGestures(
							onTap = {
								if (rootKey
										.optString("address_key")
										.isNotBlank()
								)
									button(ButtonID.SelectKey, rootKey.optString("address_key"))
							},
							onLongPress = {
								if (rootKey
										.optString("address_key")
										.isNotBlank()
								)
									button(ButtonID.LongTap, rootKey.optString("address_key"))
							}
						)
					}
					.draggable(
						state = rememberDraggableState { delta ->
							offsetX += delta
						},
						orientation = Orientation.Horizontal,
						onDragStopped = {
							if (offsetX.absoluteValue > 20f) {
								if (rootKey
										.optString("address_key")
										.isNotBlank()
								)
									button(ButtonID.Swipe, rootKey.optString("address_key"))
							}
							offsetX = 0f
						}
					)
					.padding(top = 3.dp, start = 12.dp, end = 12.dp)
					.background(MaterialTheme.colors.Bg200)
					.fillMaxWidth()
			) {
				SeedCard(
					seedName = rootKey.optString("seed_name", "error"),
					identicon = rootKey.optString("identicon"),
					base58 = rootKey.optString("base58"),
					showAddress = true,
					multiselectMode = multiselectMode,
					selected = rootKey.optBoolean("multiselect"),
					swiped = rootKey.optBoolean("swiped"),
					increment = { number ->
						increment(
							number,
							rootKey.optString("seed_name")
						)
					},
					delete = { button(ButtonID.RemoveKey, "") }
				)
			}
			Row(
				verticalAlignment = Alignment.CenterVertically,
				modifier = Modifier
					.clickable { button(ButtonID.NetworkSelector, "") }
					.padding(top = 3.dp, start = 12.dp, end = 12.dp)
					.background(MaterialTheme.colors.Bg100)
					.fillMaxWidth()
					.padding(top = 8.dp, start = 20.dp, end = 12.dp)
			) {
				screenData.optJSONObject("network")?.let { network ->
					NetworkLogoName(
						logo = network.optString("logo"),
						name = network.optString("title")
					)
				}
				Spacer(Modifier.width(8.dp))
				Icon(
					Icons.Outlined.ExpandCircleDown,
					"More networks",
					tint = MaterialTheme.colors.Action400
				)
				Spacer(modifier = Modifier.weight(1f))
			}
			Row(
				verticalAlignment = Alignment.CenterVertically,
				modifier = Modifier
					.padding(top = 3.dp, start = 12.dp, end = 12.dp)
					.fillMaxWidth(1f)
					.padding(horizontal = 8.dp)
			) {
				Text("DERIVED KEYS")
				Spacer(Modifier.weight(1f, true))
				IconButton(onClick = {
					if (alertState == ShieldAlert.None)
						button(ButtonID.NewKey, "")
					else
						button(ButtonID.Shield, "")
				}) {
					Icon(
						Icons.Default.AddCircleOutline,
						contentDescription = "New derived key",
						tint = MaterialTheme.colors.Action400
					)
				}
			}
			KeySelector(
				button,
				{ number -> increment(number, rootKey.optString("seed_name")) },
				keySet,
				multiselectMode
			)
		}
		if (multiselectMode) {
			Column {
				Spacer(Modifier.weight(1f))
				BottomMultiselectBar(
					count = multiselectCount,
					delete = { button(ButtonID.RemoveKey, "") },
					export = { button(ButtonID.ExportMultiSelect, "") }
				)
			}
		}
	}
}


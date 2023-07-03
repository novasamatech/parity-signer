package io.parity.signer.domain.storage

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringSetPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.map

val Context.dataStore: DataStore<Preferences> by preferencesDataStore(name = "app_preferences")

class PreferencesRepository(private val context: Context) {

	private val NETWORK_FILTER = stringSetPreferencesKey("network_filter")

	val networksFilter = context.dataStore.data
		.map { preferences ->
			// No type safety.
			preferences[NETWORK_FILTER] ?: emptySet()
		}

	suspend fun setNetworkFilter(newFilters: Set<String>) {
		context.dataStore.edit { settings ->
			settings[NETWORK_FILTER] = newFilters
		}
	}
}

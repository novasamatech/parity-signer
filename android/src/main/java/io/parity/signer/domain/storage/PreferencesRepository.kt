package io.parity.signer.domain.storage

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringSetPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.map

private val Context.dataStore: DataStore<Preferences> by preferencesDataStore(name = "app_preferences")

class PreferencesRepository(private val context: Context) {

	private val networksFilterKey = stringSetPreferencesKey("network_filter")

	val networksFilter = context.dataStore.data
		.map { preferences ->
			// No type safety.
			preferences[networksFilterKey] ?: emptySet()
		}

	suspend fun setNetworksFilter(newFilters: Set<String>) {
		context.dataStore.edit { settings ->
			settings[networksFilterKey] = newFilters
		}
	}
}

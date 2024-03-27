import { configureStore } from '@reduxjs/toolkit'

import CalendarSlice from './slices/CalendarSlice'
import SectorSlice from './slices/SectorSlice'
import CurrencySlice from './slices/CurrencySlice'

const store = configureStore({
  reducer: {
    [CalendarSlice.name]: CalendarSlice.reducer,
    [SectorSlice.name]: SectorSlice.reducer,
    [CurrencySlice.name]: CurrencySlice.reducer
  }
})

export default store

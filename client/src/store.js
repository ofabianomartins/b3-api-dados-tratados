import { configureStore } from '@reduxjs/toolkit'

import CalendarSlice from './slices/CalendarSlice'
import SectorSlice from './slices/SectorSlice'
import CurrencySlice from './slices/CurrencySlice'
import SubsectorSlice from './slices/SubsectorSlice'
import CompanySlice from './slices/CompanySlice'
import TickerSlice from './slices/TickerSlice'
import IndicatorSlice from './slices/IndicatorSlice'

const store = configureStore({
  reducer: {
    [CalendarSlice.name]: CalendarSlice.reducer,
    [SectorSlice.name]: SectorSlice.reducer,
    [SubsectorSlice.name]: SubsectorSlice.reducer,
    [CompanySlice.name]: CompanySlice.reducer,
    [TickerSlice.name]: TickerSlice.reducer,
    [IndicatorSlice.name]: IndicatorSlice.reducer,
    [CurrencySlice.name]: CurrencySlice.reducer
  }
})

export default store

import { configureStore } from '@reduxjs/toolkit'

import CalendarSlice from './slices/CalendarSlice'
import SectorSlice from './slices/SectorSlice'

const store = configureStore({
  reducer: {
    [CalendarSlice.name]: CalendarSlice.reducer,
    [SectorSlice.name]: SectorSlice.reducer
  }
})

export default store

import { createSlice, createAsyncThunk } from '@reduxjs/toolkit'

import server from '../server'

export default (option) => {
  const initialObject = option.initialObject || {}
  const index = createAsyncThunk(
    `${option.name}/index`,
    async () => server.get(option.path).then(response => response.data)
  )

  const create = createAsyncThunk(
    `${option.name}/create`,
    async (data) => server.post(option.path, data).then(resp => resp.data)
  )

  const destroy = createAsyncThunk(
    `${option.name}/destroy`,
    async (id) => server.delete(`${option.path}/${id}`)
  )

  const slice = createSlice({
    name: option.name,
    initialState: {
      list: [],
      obj: initialObject
    },
    reducers: {},
    extraReducers: (builder) => {
      builder.addCase(index.fulfilled, (state, action) => {
        state.list = action.payload;
      })

      builder.addCase(create.fulfilled, (state, action) => {
        state.obj = action.payload;
      })

      builder.addCase(destroy.fulfilled, (state, _action) => {
        state.obj = initialObject;
      })
    }
  })

  return {
    name: option.name,
    reducer: slice.reducer,
    actions: { ...slice.actions, index, create, destroy }
  }
}


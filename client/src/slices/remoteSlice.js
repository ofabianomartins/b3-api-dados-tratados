import { createSlice, createAsyncThunk } from '@reduxjs/toolkit'

import server from '../server'

export default (option) => {
  const initialObject = option.initialObject || {}

  const index = () => {
    return async (dispatch) => {
      return server.get(`${option.path}`)
        .then(resp => dispatch({ type: `${option.name}/setList`, payload: resp.data }))
    }
  }

  const show = (id) => {
    return async (dispatch) => {
      return server.get(`${option.path}/${id}`)
        .then(resp => dispatch({ type: `${option.name}/setBody`, payload: resp.data }))
    }
  }

  const create = (data) => {
    return async (dispatch) => {
      return server.post(option.path, data)
        .then(resp => dispatch({ type: `${option.name}/setBody`, payload: resp.data }))
    }
  }

  const update = (id, data) => {
    return async (dispatch) => {
      return server.put(`${option.path}/${id}`, data)
        .then(resp => dispatch({ type: `${option.name}/setBody`, payload: resp.data }))
    }
  }

  const destroy = (id) => {
    return async (dispatch) => {
      return server.delete(`${option.path}/${id}`)
        .then(() => dispatch({ type: `${option.name}/setBody`, payload: initialObject }))
    }
  }

  const slice = createSlice({
    name: option.name,
    initialState: {
      list: [],
      obj: initialObject
    },
    reducers: {
      setList: (state, action) => {
        state.list = action.payload;
      },
      setBody: (state, action) => {
        state.obj = action.payload;
      }
    },
    extraReducers: (builder) => {  }
  })

  return {
    name: option.name,
    reducer: slice.reducer,
    actions: { ...slice.actions, index, show, create, update, destroy }
  }
}


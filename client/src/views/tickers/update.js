import React, { useEffect, useState } from 'react'
import { CAlert } from '@coreui/react'

import { useDispatch, useSelector } from 'react-redux'
import TickerSlice from '../../slices/TickerSlice'

import Form from './_form'
import { useParams } from 'react-router-dom'

export default () => {
  const params = useParams()

  const obj = useSelector(state => state.tickers.obj);
  const [data, setData] = useState({})
  const [created, setCreated] = useState(false)
  const dispatch = useDispatch()

  useEffect(() => {
    dispatch(TickerSlice.actions.show(params.id))
  }, [])

  useEffect(() => { setData(obj) }, [obj])

  const handleSave = () => {
    dispatch(TickerSlice.actions.update(params.id, data))
      .then(() => setCreated(true))
  }

  return (
    <React.Fragment>
      {created && <CAlert color="success"> Ticker {params.id} updated! </CAlert>}
      <Form
        handleSave={handleSave}
        data={data}
        updateAttr={(attr, value) => setData((previous) => ({ ...previous, [attr]: value }))}
      />
    </React.Fragment>
  )
}


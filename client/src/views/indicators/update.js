import React, { useEffect, useState } from 'react'
import { CAlert } from '@coreui/react'

import { useDispatch, useSelector } from 'react-redux'
import IndicatorSlice from '../../slices/IndicatorSlice'

import Form from './_form'
import { useParams } from 'react-router-dom'

export default () => {
  const params = useParams()

  const obj = useSelector(state => state.indicators.obj);
  const [data, setData] = useState({})
  const [created, setCreated] = useState(false)
  const dispatch = useDispatch()

  useEffect(() => {
    dispatch(IndicatorSlice.actions.show(params.id))
  }, [])

  useEffect(() => { setData(obj) }, [obj])

  const handleSave = () => {
    dispatch(IndicatorSlice.actions.update(params.id, data))
      .then(() => setCreated(true))
  }

  return (
    <React.Fragment>
      {created && <CAlert color="success"> Indicator {params.id} updated! </CAlert>}
      <Form
        handleSave={handleSave}
        data={data}
        updateAttr={(attr, value) => setData((previous) => ({ ...previous, [attr]: value }))}
      />
    </React.Fragment>
  )
}


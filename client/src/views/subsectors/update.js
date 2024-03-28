import React, { useEffect, useState } from 'react'
import { CAlert } from '@coreui/react'
import { useParams } from 'react-router-dom'
import { useDispatch, useSelector } from 'react-redux'

import SubsectorSlice from '../../slices/SubsectorSlice'

import Form from './_form'

export default () => {
  const params = useParams()

  const obj = useSelector(state => state.currencies.obj);
  const [data, setData] = useState({})
  const [created, setCreated] = useState(false)
  const dispatch = useDispatch()

  useEffect(() => {
    dispatch(SubsectorSlice.actions.show(params.id))
  }, [])

  useEffect(() => { setData(obj) }, [obj])

  const handleSave = () => {
    dispatch(SubsectorSlice.actions.update(params.id, data))
      .then(() => setCreated(true))
  }

  return (
    <React.Fragment>
      {created && <CAlert color="success"> Subsector {params.id} updated! </CAlert>}
      <Form
        handleSave={handleSave}
        data={data}
        updateAttr={(attr, value) => setData((previous) => ({ ...previous, [attr]: value }))}
      />
    </React.Fragment>
  )
}


import React, { useState } from 'react'
import { CAlert } from '@coreui/react'
import { useDispatch } from 'react-redux'

import SubsectorSlice from '../../slices/SubsectorSlice'

import Form from './_form'

export default () => {
  const [data, setData] = useState({})
  const [created, setCreated] = useState(false)
  const dispatch = useDispatch()

  const handleSave = () => {
    dispatch(SubsectorSlice.actions.create(data))
      .then(() => setCreated(true))
  }

  return (
    <React.Fragment>
      {created && <CAlert color="success"> Subsector create! </CAlert>}
      <Form
        handleSave={handleSave}
        data={data}
        updateAttr={(attr, value) => setData((previous) => ({ ...previous, [attr]: value }))}
      />
    </React.Fragment>
  )
}


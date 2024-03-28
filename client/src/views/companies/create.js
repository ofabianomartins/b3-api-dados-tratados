import React, { useState } from 'react'
import { CAlert } from '@coreui/react'

import { useDispatch } from 'react-redux'
import CompanySlice from '../../slices/CompanySlice'

import Form from './_form'

export default () => {
  const [data, setData] = useState({})
  const [created, setCreated] = useState(false)
  const dispatch = useDispatch()

  const handleSave = () => {
    dispatch(CompanySlice.actions.create(data))
      .then(() => setCreated(true))
  }

  return (
    <React.Fragment>
      {created && <CAlert color="success"> Company create! </CAlert>}
      <Form
        handleSave={handleSave}
        data={data}
        updateAttr={(attr, value) => setData((previous) => ({ ...previous, [attr]: value }))}
      />
    </React.Fragment>
  )
}


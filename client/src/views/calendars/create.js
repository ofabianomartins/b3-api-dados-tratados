import React, { useState } from 'react'
import {
  CCol,
  CButton,
  CForm,
  CFormInput,
  CFormLabel,
  CAlert,
  CRow,
} from '@coreui/react'

import { useDispatch } from 'react-redux'
import CalendarSlice from '../../slices/CalendarSlice'

export default () => {
  const [data, setData] = useState({})
  const [created, setCreated] = useState(false)
  const dispatch = useDispatch()

  const updateAttr = (attr, value) => setData((previous) => ({ ...previous, [attr]: value }))

  const handleSave = () => {
    dispatch(CalendarSlice.actions.create(data))
      .then(() => setCreated(true))
  }

  return (
    <CRow>
      <CCol xs={12}>
        {created && <CAlert color="success"> Calendar create! </CAlert>}
        <CForm>
          <div className="mb-3">
            <CFormLabel htmlFor="exampleFormControlInput1">Name</CFormLabel>
            <CFormInput
              type="text"
              id="exampleFormControlInput1"
              placeholder="Calendar1"
              value={data.name || ""}
              onChange={(event) => updateAttr('name', event.target.value)}
            />
          </div>

          <div className="mb-3">
            <CFormLabel htmlFor="exampleFormControlInput2">Code</CFormLabel>
            <CFormInput
              type="text"
              id="exampleFormControlInput2"
              placeholder="Code1"
              value={data.code || ""}
              onChange={(event) => updateAttr('code', event.target.value)}
            />
          </div>

          <CButton
            color="info"
            variant="outline"
            onClick={() => handleSave()}
          >Create</CButton>
        </CForm>
      </CCol>
    </CRow>
  )
}


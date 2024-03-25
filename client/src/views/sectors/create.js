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
import SectorSlice from '../../slices/SectorSlice'

export default () => {
  const [data, setData] = useState({})
  const [created, setCreated] = useState(false)
  const dispatch = useDispatch()

  const updateAttr = (attr, value) => setData((previous) => ({ ...previous, [attr]: value }))

  const handleSave = () => {
    dispatch(SectorSlice.actions.create(data))
      .then(() => setCreated(true))
  }

  return (
    <CRow>
      <CCol xs={12}>
        {created && <CAlert color="success"> Sector create! </CAlert>}
        <CForm>
          <div className="mb-3">
            <CFormLabel htmlFor="exampleFormControlInput1">Name</CFormLabel>
            <CFormInput
              type="text"
              id="exampleFormControlInput1"
              placeholder="Sector1"
              value={data.name || ""}
              onChange={(event) => updateAttr('name', event.target.value)}
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


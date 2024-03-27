import React, { useState } from 'react'
import {
  CCol,
  CButton,
  CForm,
  CFormInput,
  CFormLabel,
  CRow,
} from '@coreui/react'

export default ({ handleSave, updateAttr, data, disable }) => {
  return (
    <CRow>
      <CCol xs={12}>
        <CForm>
          <div className="mb-3">
            <CFormLabel htmlFor="exampleFormControlInput1">Name</CFormLabel>
            <CFormInput
              type="text"
              id="exampleFormControlInput1"
              placeholder="Calendar1"
              disabled={disable}
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
              disabled={disable}
              value={data.code || ""}
              onChange={(event) => updateAttr('code', event.target.value)}
            />
          </div>

          { !disable && <CButton
            color="info"
            variant="outline"
            onClick={() => handleSave()}
          >Create</CButton>}
        </CForm>
      </CCol>
    </CRow>
  )
}


import React, { useEffect } from 'react'
import {
  CCol,
  CButton,
  CForm,
  CFormInput,
  CFormLabel,
  CFormSelect,
  CRow,
} from '@coreui/react'
import { useDispatch, useSelector } from 'react-redux'

import SectorSlice from '../../slices/SectorSlice'

export default ({ handleSave, updateAttr, data, disable }) => {
  const listSectors = useSelector(state => state.sectors.list);
  const dispatch = useDispatch();

  useEffect(() => {
    dispatch(SectorSlice.actions.index());
  },[])

  return (
    <CRow>
      <CCol xs={12}>
        <CForm>
          <div className="mb-3">
            <CFormLabel htmlFor="exampleFormControlInput1">Name</CFormLabel>
            <CFormInput
              type="text"
              id="exampleFormControlInput1"
              placeholder="Subsector1"
              disabled={disable}
              value={data.name || ""}
              onChange={(event) => updateAttr('name', event.target.value)}
            />
          </div>

          <div className="mb-3">
            <CFormLabel htmlFor="exampleFormControlInput2">Code</CFormLabel>
            <CFormSelect
              aria-label="Default select example"
              options={[
                'Open this select menu',
                ...listSectors.map(elem => ({ label: elem.name, value: elem.id }))
              ]}
              onChange={(event) => updateAttr('sector_id', parseInt(event.target.value))}
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


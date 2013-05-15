/* -------------------------------------------------------------------------- *
 *                              OpenMMAmoeba                                  *
 * -------------------------------------------------------------------------- *
 * This is part of the OpenMM molecular simulation toolkit originating from   *
 * Simbios, the NIH National Center for Physics-Based Simulation of           *
 * Biological Structures at Stanford, funded under the NIH Roadmap for        *
 * Medical Research, grant U54 GM072970. See https://simtk.org.               *
 *                                                                            *
 * Portions copyright (c) 2011 Stanford University and the Authors.           *
 * Authors: Peter Eastman                                                     *
 * Contributors:                                                              *
 *                                                                            *
 * This program is free software: you can redistribute it and/or modify       *
 * it under the terms of the GNU Lesser General Public License as published   *
 * by the Free Software Foundation, either version 3 of the License, or       *
 * (at your option) any later version.                                        *
 *                                                                            *
 * This program is distributed in the hope that it will be useful,            *
 * but WITHOUT ANY WARRANTY; without even the implied warranty of             *
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the              *
 * GNU Lesser General Public License for more details.                        *
 *                                                                            *
 * You should have received a copy of the GNU Lesser General Public License   *
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.      *
 * -------------------------------------------------------------------------- */

#include "ReferenceDrudeKernelFactory.h"
#include "ReferenceDrudeKernels.h"
#include "ReferencePlatform.h"
#include "openmm/internal/ContextImpl.h"
#include "openmm/OpenMMException.h"

using namespace OpenMM;

#if defined(WIN32)
    #include <windows.h>
    extern "C" void initDrudeReferenceKernels();
    BOOL WINAPI DllMain(HANDLE hModule, DWORD  ul_reason_for_call, LPVOID lpReserved) {
        if (ul_reason_for_call == DLL_PROCESS_ATTACH)
            initDrudeReferenceKernels();
        return TRUE;
    }
#else
    extern "C" void __attribute__((constructor)) initDrudeReferenceKernels();
#endif

extern "C" void initDrudeReferenceKernels() {
    Platform& platform = Platform::getPlatformByName("Reference");
    ReferenceDrudeKernelFactory* factory = new ReferenceDrudeKernelFactory();
    platform.registerKernelFactory(CalcDrudeForceKernel::Name(), factory);
    platform.registerKernelFactory(IntegrateDrudeLangevinStepKernel::Name(), factory);
}

KernelImpl* ReferenceDrudeKernelFactory::createKernelImpl(std::string name, const Platform& platform, ContextImpl& context) const {
    ReferencePlatform::PlatformData& data = *static_cast<ReferencePlatform::PlatformData*>(context.getPlatformData());
    if (name == CalcDrudeForceKernel::Name())
        return new ReferenceCalcDrudeForceKernel(name, platform);
    if (name == IntegrateDrudeLangevinStepKernel::Name())
        return new ReferenceIntegrateDrudeLangevinStepKernel(name, platform, data);
    throw OpenMMException((std::string("Tried to create kernel with illegal kernel name '")+name+"'").c_str());
}
